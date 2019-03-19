fn main() {
    println!("Enter an emoji name!");

    let mut word = String::new();
    std::io::stdin().read_line(&mut word)
        .expect("Failed to read line");

    // Trim all whitespace
    let word = word.trim();

    println!("Searching for emoji {} \u{1F50D}", word);
    if word == "dog" {
        println!("\u{1F415}   (U+1F415)");
    } else if word == "cat" {
        println!("\u{1F63A}   (U+1F63A)");
    } else if word == "ghost" {
        println!("\u{1F47B}   (U+1F47B)");
    } else if word == "canada" {
        println!("\u{1F1E8}\u{1F1E6}   (U+1F1E8 U+1F1E6)");
    } else {
        println!("Uh oh, no emoji found for: {}", word);
    }
}
