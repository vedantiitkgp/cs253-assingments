use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use std::iter::FromIterator;

// Shared mutable data
static mut DATA: Vec<char> = Vec::new();
static mut WORDS: Vec<String> = Vec::new();
static mut WORD_FREQS: Vec<(String, usize)> = Vec::new();

// Procedures

// Function to read the entire file content into DATA
fn read_file(path: &str) -> io::Result<()> {
    unsafe {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        DATA = content.chars().collect();
    }
    Ok(())
}

// Function to normalize data and replace non-alphanumeric characters with spaces
fn filter_chars_and_normalize() {
    unsafe {
        for i in 0..DATA.len() {
            if !DATA[i].is_alphanumeric() {
                DATA[i] = ' ';
            } else {
                DATA[i] = DATA[i].to_ascii_lowercase();
            }
        }
    }
}

// Function to scan and split DATA into words and store them in WORDS
fn scan() {
    unsafe {
        let data_str: String = DATA.iter().collect();
        WORDS = data_str.split_whitespace().map(|s| s.to_string()).collect();
    }
}

// Function to remove stop words from WORDS
fn remove_stop_words() -> io::Result<()> {
    let mut stop_words_content = String::new();
    let mut file = File::open("../stop_words.txt")?;
    file.read_to_string(&mut stop_words_content)?;

    let mut stop_words: Vec<String> = stop_words_content.split(',').map(|s| s.to_string()).collect();
    stop_words.extend("abcdefghijklmnopqrstuvwxyz".chars().map(|c| c.to_string()));

    unsafe {
        WORDS.retain(|word| !stop_words.contains(word));
    }
    Ok(())
}

// Function to compute frequencies of words
fn frequencies() {
    unsafe {
        let mut word_freqs: HashMap<String, usize> = HashMap::new();
        for word in &WORDS {
            *word_freqs.entry(word.clone()).or_insert(0) += 1;
        }
        WORD_FREQS = Vec::from_iter(word_freqs.into_iter());
    }
}

// Function to sort word frequencies in descending order
fn sort() {
    unsafe {
        WORD_FREQS.sort_by(|a, b| b.1.cmp(&a.1));
    }
}

// The main function
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_process>", args[0]);
        std::process::exit(1);
    }

    // Execute each procedure sequentially
    read_file(&args[1])?;
    filter_chars_and_normalize();
    scan();
    remove_stop_words()?;
    frequencies();
    sort();

    // Print the top 25 words
    unsafe {
        for tf in WORD_FREQS.iter().take(25) {
            println!("{} - {}", tf.0, tf.1);
        }
    }

    Ok(())
}