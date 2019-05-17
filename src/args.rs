use super::Command;

use atty::Stream;

use std::io;

use clap::{crate_authors, crate_description, crate_name, crate_version};
use clap::{App, Arg, SubCommand};

fn get_stdin() -> String {
    if atty::is(Stream::Stdin) {
        "".to_string()
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

pub(super) fn parse() -> Command {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("get")
                .about("Get unicode emoji given a name (via arg or stdin)")
                .arg(
                    Arg::with_name("name")
                        .help("Name of the emoji to display")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("preview")
                .about("Display a list of all available emojis by name"),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(m)) => match m.value_of("name") {
            Some(name) => Command::Get(name.to_string()),
            _ => Command::Get(get_stdin()),
        },
        ("preview", _) => Command::Preview,
        _ => std::process::exit(0),
    }
}
