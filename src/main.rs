use std::io;

fn pig_latin(english: &str) -> String {
    let mut output = String::new();
    for word in english.to_lowercase().split_whitespace() {
        let (first_letter, rest_of_word) = word.split_at(1);
        match b"aeiou".contains(&first_letter.as_bytes()[0]) {
            true => output.push_str(&format!("{}-hay ", english)),
            false => output.push_str(&format!("{}-{}ay ", rest_of_word, first_letter)),
        }
    }
    output
}

fn main() {
    println!("English to translate: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed.");

    println!("Pig Latin: {}", pig_latin(&input));
}
