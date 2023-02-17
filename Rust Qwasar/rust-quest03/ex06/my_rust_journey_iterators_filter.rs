use std::env;

fn main() {
match env::args().nth(1) {
Some(sentence) => {
let words: Vec<&str> = sentence
.split_whitespace()
.collect();
    let words_containing_t: Vec<&str> = words
        .into_iter()
        .filter(|word| word.contains("t"))
        .collect();
    println!("{:?}", words_containing_t);
  }
  None => {}
}
}