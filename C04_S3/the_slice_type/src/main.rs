fn main() {
    let phrase1 = String::from("Hello World|");
    let word1 = first_word(&phrase1);
    println!("{}", word1);

    let phrase2 = String::from("Phalenopsis amabalis");
    let word2 = first_slice(&phrase2);
    println!("{}", word2);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// Extract the first word of a phrase with slices, if there is only a single word return the word.
fn first_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}