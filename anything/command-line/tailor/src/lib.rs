use crate::TakeValue::*;
use clap::{App, Arg};
use std::fs::File;
use std::path::Path;
use std::{
    error::Error,
    io::{BufRead, BufReader, Read, Seek},
};

use once_cell::sync::OnceCell;
use regex::Regex;

use notify::{RecommendedWatcher, RecursiveMode, Watcher};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: TakeValue,
    bytes: Option<TakeValue>,
    quiet: bool,
    follow: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("tailr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust tail")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Suppress headers"),
        )
        .arg(
            Arg::with_name("follow")
                .short("f")
                .long("follow")
                .help("Follow file"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_num)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes: bytes,
        quiet: matches.is_present("quiet"),
        follow: matches.is_present("follow"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:?}", config);
    for filename in config.files {
        match File::open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let (total_lines, _total_bytes) = count_lines_bytes(&filename)?;
                // println!("{}: {} lines", filename, total_lines);
                let file = BufReader::new(file);
                print_lines(file, &config.lines, total_lines)?;

                if config.follow {
                    watch(filename, total_lines)?;
                }
            }
        }
    }
    Ok(())
}

// watch a file for changes using notify crate
// https://docs.rs/notify/latest/notify/
// sample: https://github.com/notify-rs/notify/blob/08e74dae8e96fbd25704cdaa530ffc02f6d33039/examples/monitor_raw.rs
fn watch<P: AsRef<Path>>(path: P, start: u64) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    let filename = path.as_ref().to_str().unwrap();

    let mut next = start;
    for res in rx {
        match res {
            Ok(event) => {
                // println!("{:?}", event);
                let (total_lines, _total_bytes) = count_lines_bytes(filename).unwrap();
                // prevent printing the same lines over and over.
                if total_lines <= next {
                    next = total_lines;
                    continue;
                }

                // File should open and close each time.
                let file = File::open(filename)?;
                print_lines(BufReader::new(&file), &TakeNum(1), total_lines)
                    .expect("failed to follow file");

                next = total_lines;
            }
            Err(error) => {
                println!("error: {:?}", error);
            }
        }
    }

    Ok(())
}

fn parse_num(val: &str) -> MyResult<TakeValue> {
    let num_re = Regex::new(r"^([+-])?(\d+)$").unwrap();

    match num_re.captures(val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("+", |m| m.as_str());
            let num = format!("{}{}", sign, caps.get(2).unwrap().as_str());
            if let Ok(val) = num.parse() {
                if sign == "+" && val == 0 {
                    Ok(PlusZero)
                } else {
                    // println!("val = {}", val);
                    Ok(TakeNum(val))
                }
            } else {
                Err(From::from(val))
            }
        }
        _ => Err(From::from(val)),
    }
}

static NUM_RE: OnceCell<Regex> = OnceCell::new();

fn count_lines_bytes(filename: &str) -> MyResult<(u64, i64)> {
    let num_re = NUM_RE.get_or_init(|| Regex::new(r"(\d+)").unwrap());
    let mut total_lines = 0;
    let mut total_bytes = 0;
    let file = File::open(filename)?;
    for line in std::io::BufReader::new(file).lines() {
        let line = line?;
        total_lines += 1;
        total_bytes += line.len() as i64;
        for cap in num_re.captures_iter(&line) {
            if let Ok(val) = cap[1].parse::<i64>() {
                total_bytes += val;
            }
        }
    }
    Ok((total_lines, total_bytes))
}

fn print_lines(mut file: impl BufRead, num_lines: &TakeValue, total_lines: u64) -> MyResult<()> {
    if let Some(start) = get_start_index(num_lines, total_lines) {
        // println!("start = {}", start);
        let mut line_num = 0;
        let mut buf = Vec::new();
        loop {
            let bytes_read = file.read_until(b'\n', &mut buf)?;
            if bytes_read == 0 {
                break;
            }
            if line_num >= start {
                print!("{}", String::from_utf8_lossy(&buf));
            }
            line_num += 1;
            buf.clear();
        }
    } else {
        eprintln!("illegal line count -- {:?}", num_lines);
    }

    Ok(())
}

fn get_start_index(take_val: &TakeValue, total: u64) -> Option<u64> {
    match take_val {
        PlusZero => Some(0),
        TakeNum(n) => {
            // println!("n = {}", n);
            if *n > 0 {
                // let t = total.saturating_sub(*n as u64);
                Some(total.saturating_sub(*n as u64))
            } else {
                None
            }
        }
    }
}

fn print_bytes<T: Read + Seek>(
    mut file: T,
    num_bytes: &TakeValue,
    total_bytes: i64,
) -> MyResult<()> {
    unimplemented!();
}

// fn print_bytes<T>(
//     mut file: T,
//     num_bytes: &TakeValue,
//     total_bytes: i64,
// ) -> MyResult<()>
// where
//     T: Read + Seek,
// {
//     unimplemented!();
// }
