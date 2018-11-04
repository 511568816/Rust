use std::io;

fn slice_word (sentence: &str) -> &str {
    let sentence_bytes = sentence.as_bytes();
    for (i, &c) in sentence_bytes.iter().enumerate() {
        if c == b' ' {
            return &sentence[..i];
        }
    }
    &sentence
}

fn main() {
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence).expect("Fail to read");
    let first_word = slice_word(&sentence[..]);
    println!("{}", first_word);
}
