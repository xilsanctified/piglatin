fn swap(word: &str) -> String {
    match word.to_lowercase().chars().next() {
        Some('a' | 'e' | 'i' | 'o' | 'u') => String::from(word.to_string()) + "-hay ",
        _ => consonant(word),
    }
}

fn pig_latin(s: &str) -> String {
    if s.is_empty() {
        return String::from("Empty string!");
    }
    let mut translation = String::new();
    for word in s.split_whitespace() {
        translation.push_str(&swap(word));
    }
    translation
}

fn consonant(s: &str) -> String {
    let (first, last) = s.split_at(s.len() - s.len() + 1);
    let formatted = format!("{}{}-ay ", last.to_lowercase(), first.to_lowercase());
    formatted
}
fn main() {
    let s = "This is a test of the emergency broadcast system";
    println!("Original:  {}", s);
    println!("Pig Latin: {}", pig_latin(s));
}
