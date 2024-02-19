use chrono::{Datelike, NaiveDate};
use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    month: Option<u32>,
    year: i32,
    today: NaiveDate,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("calr")
        .version("0.1.0")
        .about("Rust cal")
        // What goes here?
        .arg(
            Arg::with_name("month")
                .short("m")
                .long("month")
                .value_name("MONTH")
                .help("Month (1-12)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .value_name("YEAR")
                .help("Year")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("today")
                .short("t")
                .long("today")
                .value_name("TODAY")
                .help("Today's date")
                .takes_value(true),
        )
        .get_matches();

    Ok(Config {
        month: matches.value_of("month").map(|m| m.parse().unwrap()),
        year: matches
            .value_of("year")
            .map(|y| y.parse().unwrap())
            .unwrap_or_else(|| chrono::Local::now().year()),
        today: matches
            .value_of("today")
            .map(|t| NaiveDate::parse_from_str(t, "%Y-%m-%d").unwrap())
            .unwrap_or_else(|| chrono::Local::now().date_naive()),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}
