use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("It's me!")
        .about("echo by Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do NOT print newline")
                .takes_value(false),
        )
        .get_matches();

    // {:#?} ってなんだっけ => pretty-printing
    // https://doc.rust-lang.org/std/fmt/#pretty-printing
    // println!("{:#?}", matches);

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    // let mut ending = "\n";
    // if omit_newline {
    //     ending = "";
    // }
    // u can write like this because if is an expression, not a statement in Rust.
    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending)
}
