mod args;
mod emojis;

enum Command {
    Get(String),
    Preview,
}

fn main() {
    let command = args::parse();

    match command {
        Command::Get(name) => display_emoji(&name),
        Command::Preview => preview_emojis(),
    }
}

fn display_emoji(name: &str) {
    for emoji in emojis::EMOJIS {
        if name == emoji.0 {
            println!("{}", emoji.1);
            std::process::exit(0);
        }
    }
    std::process::exit(1);
}

fn preview_emojis() {
    for emoji in emojis::EMOJIS {
        println!("{}", emoji.0);
    }
}
