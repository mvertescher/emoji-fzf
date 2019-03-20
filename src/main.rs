mod emojis;

fn main() {
    println!("Enter an emoji name!");

    let mut word = String::new();
    std::io::stdin().read_line(&mut word)
        .expect("Failed to read line");
    let word = word.trim();

    println!("Searching for emoji {} \u{1F50D}", word);

    for emoji in emojis::EMOJIS {
        if word == emoji.0 {
            println!("{}", emoji.1);
            return;
        }
    }
    println!("Uh oh, no emoji found for: {}", word);
}
