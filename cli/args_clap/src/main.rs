use clap::{builder::Str, command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
        .about("about? why this is not help?")
        // .group(
        //     ArgGroup::new("person-register")
        //         .arg("firstname")
        //         .arg("lastname"),
        // )
        .arg(
            // according to short method,
            // positional parameters -> named parameters
            Arg::new("projectid")
                .long("project-id")
                .aliases(["projectid", "pid"])
                .required(true)
                // only can use one of lastname or firstname
                // .conflicts_with("lastname"),
                .help("your cloud's project id"),
        )
        .group(
            // how to prevent this line from collapsing into a single line...
            ArgGroup::new("animal-register").arg("pet-name"),
        )
        .arg(Arg::new("pet-name").long("pet-name").short('n'))
        .arg(
            Arg::new("fluffy")
                .long("fluffy")
                .help("what is fluffy.....?"),
        )
        .subcommand(
            Command::new("register-person")
                .arg(
                    // according to short method,
                    // positional parameters -> named parameters
                    Arg::new("firstname")
                        .short('f')
                        .long("first-name")
                        .aliases(["fname", "firstname"])
                        .required(true)
                        // only can use one of lastname or firstname
                        // .conflicts_with("lastname"),
                        .help("your first name"),
                )
                .arg(
                    Arg::new("lastname")
                        .short('l')
                        .long("last-name")
                        .aliases(["lname"])
                        .required(true)
                        .help("your last name"),
                ),
        )
        .subcommand(
            Command::new("delete-person").arg(
                // according to short method,
                // positional parameters -> named parameters
                Arg::new("personid")
                    .long("person-id")
                    .aliases(["personid"])
                    .required(true)
                    .help("person id"),
            ),
        )
        .get_matches();

    println!(
        "fluffy {}",
        match_result.get_one::<String>("fluffy").unwrap()
    );

    let register_args = match_result.subcommand_matches("register-person");
    println!(
        "does include {}",
        register_args.unwrap().get_one::<String>("first-name").unwrap()
        // register_args.unwrap().contains_id("first-name")
    )
}
