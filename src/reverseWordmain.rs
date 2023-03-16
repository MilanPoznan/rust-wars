use std::{fmt::format, vec};

// Complete the function that accepts a string parameter, and reverses each word in the string. All spaces in the string should be retained.

// Examples
// "This is an example!" ==> "sihT si na !elpmaxe"
//"double  spaces"      ==> "elbuod  secaps"

fn reverse_words(str: &str) -> String {
    // your code here
    let word_vec = str.split(" ");

    let mut fin_vec = word_vec.into_iter().fold(String::new(), |mut acc, word| {
        let reversed_word: String = word.chars().rev().collect();
        acc = format!("{} {}", acc, reversed_word);
        acc
    });

    fin_vec.remove(0);
    fin_vec
}

fn reverse_words2(str: &str) -> String {
    str.to_string()
        .split(" ")
        .map(|sub| sub.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let fin = reverse_words("double  spaced  words");
    println!("{:?}", fin);
}
