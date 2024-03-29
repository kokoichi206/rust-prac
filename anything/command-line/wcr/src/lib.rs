use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                // set to false to read from stdin
                .required(false)
                .min_values(1),
        )
        .arg(
            Arg::with_name("lines")
                .short("l")
                .long("lines")
                .help("Count lines"),
        )
        .arg(
            Arg::with_name("words")
                .short("w")
                .long("words")
                .help("Count words"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .help("Count bytes"),
        )
        .arg(
            Arg::with_name("chars")
                .short("m")
                .long("chars")
                .help("Count chars"),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    // itterator::all
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.all
    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap_or_default(),
        lines: lines,
        words: words,
        bytes: bytes,
        chars: chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    if config.files.is_empty() {
        // when no filename is given, read from stdin
        let info = count(BufReader::new(io::stdin()))?;
        // NO filename
        // echo hoge | wc
        //    1       1       5
        print_if_needed(&config, &info, "");
        return Ok(());
    }
    let mut total = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => {
                let info = count(open(filename)?)?;
                print_if_needed(&config, &info, filename);

                total.num_lines += info.num_lines;
                total.num_words += info.num_words;
                total.num_bytes += info.num_bytes;
                total.num_chars += info.num_chars;
            }
        }
    }
    if config.files.len() > 1 {
        print_if_needed(&config, &total, "total");
    }
    Ok(())
}

fn print_if_needed(config: &Config, info: &FileInfo, filename: &str) {
    if config.lines {
        print!("{:>8}", info.num_lines);
    }
    if config.words {
        print!("{:>8}", info.num_words);
    }
    if config.bytes {
        print!("{:>8}", info.num_bytes);
    }
    if config.chars {
        print!("{:>8}", info.num_chars);
    }
    if filename.is_empty() {
        println!();
    } else {
        println!(" {}", filename);
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// file is a BufRead trait object.
pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut buf = String::new();
    loop {
        let bytes_read = file.read_line(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        if buf.ends_with('\n') {
            num_lines += 1;
        }
        num_bytes += bytes_read;
        num_chars += buf.chars().count();
        num_words += buf.split_whitespace().count();
        buf.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }
}
