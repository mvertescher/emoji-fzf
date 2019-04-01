use super::Command;

use clap::{crate_name, crate_description, crate_version, crate_authors};
use clap::{App, Arg, SubCommand};

pub(super) fn parse() -> Command {
	let matches = App::new(crate_name!())
		.about(crate_description!())
		.version(crate_version!())
		.author(crate_authors!())
		.subcommand(SubCommand::with_name("get")
					.about("Get unicode emoji given a name")
					.arg(Arg::with_name("name")
						 .help("Name of the emoji to display")
						 .index(1)
						 .required(true)))
		.subcommand(SubCommand::with_name("preview")
					.about("Display a list of all available emojis by name"))
		.get_matches();

	match matches.subcommand() {
		("get", Some(m)) => {
			Command::Get(m.value_of("name").unwrap().to_string())
		}
		("preview", _) => {
			Command::Preview
		}
		_ => std::process::exit(0),
	}
}
