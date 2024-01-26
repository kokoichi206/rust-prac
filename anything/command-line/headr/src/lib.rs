use clap::{App, Arg};
use std::error::Error;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .short("n")
                .long("lines")
                .help("Number of lines")
                .default_value("10")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .short("c")
                .long("bytes")
                .help("Number of bytes"),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap_or_default(),
        lines: matches.value_of("lines").unwrap_or("10").parse()?,
        bytes: matches.value_of("bytes").map(|n| n.parse()).transpose()?,
    })
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    // unimplemented!();
    match val.parse() {
        Ok(n) => {
            if n > 0 {
                Ok(n)
            } else {
                Err(From::from(val))
            }
        }
        _ => Err(From::from(val)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    // println!("{:#?}", config);
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                let mut line = String::new();
                if let Some(num_bytes) = config.bytes {
                    // TODO:
                    eprintln!("bytes not implemented");
                } else {
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}
