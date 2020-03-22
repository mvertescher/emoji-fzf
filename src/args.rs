//! Command argument parsing

use atty::Stream;
use std::io;
use structopt::StructOpt;
use super::Command;

fn get_stdin() -> String {
    if atty::is(Stream::Stdin) {
        "".to_string()
    } else {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[derive(Debug, StructOpt)]
#[structopt()]
enum Opt {
    #[structopt(about = "Get unicode emoji given a name (via arg or stdin).")]
    Get {
        #[structopt(help = "Name of the emoji to display.")]
        name: Option<String>,
    },
    #[structopt(about = "Display a list of all available emojis by name.")]
    Preview,
}

pub(super) fn parse() -> Command {
    let opt = Opt::from_args();

    match opt {
        Opt::Get { name: None } => Command::Get(get_stdin()),
        Opt::Get { name: Some(n) } => Command::Get(n),
        Opt::Preview => Command::Preview,
    }
}
