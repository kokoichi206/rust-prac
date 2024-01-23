use clap::{App, Arg};
use std::error::Error;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    squeeze_blank: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .required(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("With number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("non_brank")
                .short("b")
                .help("Number the non-blank output lines, starting at 1")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("squeeze_blank")
                .short("s")
                .help("Suppress repeated empty output lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("non_brank"),
        squeeze_blank: matches.is_present("squeeze_blank"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // dbg!(config);
    // Ok(())
    println!("{:?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut line_number = 1;
                let mut last_blank = false;
                // for (line_number, line_result) in file.lines().enumerate() {
                for line_result in file.lines() {
                    let line = line_result?;

                    if config.squeeze_blank && line.is_empty() && last_blank {
                        continue;
                    }

                    if config.number_lines {
                        // 6 characters wide, right justified
                        // < for left justified, ^ for centered
                        println!("{:>6}\t{} ", line_number, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("{}", line);
                        } else {
                            println!("{:>6}\t{} ", line_number, line);
                        }
                    } else {
                        println!("{}", line);
                    }

                    update(
                        line,
                        &mut line_number,
                        &mut last_blank,
                        config.number_lines,
                        config.number_nonblank_lines,
                        config.squeeze_blank,
                    )
                }
            }
        }
    }
    Ok(())
}

fn update(
    line: String,
    line_number: &mut i32,
    last_blank: &mut bool,
    flag_n: bool,
    flag_b: bool,
    flag_s: bool,
) {
    if flag_s {
        *last_blank = line.is_empty();
    }
    if flag_n {
        *line_number += 1;
    }
    if flag_b && !line.is_empty() {
        *line_number += 1;
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
