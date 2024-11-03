use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::io::{self, Result, Error, ErrorKind};

// Function to read the contents of a file and return it as a string
fn read_file(path: &str) -> Result<String> {
    if path.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "File path is empty."));
    }
    fs::read_to_string(path).map_err(|e| {
        eprintln!("Error reading file '{}': {}", path, e);
        e
    })
}

// Function to filter non-alphanumeric characters and normalize to lowercase
fn filter_chars_and_normalize(text: &str) -> String {
    if text.is_empty() {
        eprintln!("Warning: Provided text is empty.");
    }
    text.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { ' ' })
        .collect()
}

// Function to split the normalized text into words
fn scan(text: &str) -> Vec<String> {
    if text.is_empty() {
        eprintln!("Warning: No text to scan for words.");
    }
    text.split_whitespace().map(|s| s.to_string()).collect()
}

// Function to remove stop words from a list of words
fn remove_stop_words(words: Vec<String>) -> Result<Vec<String>> {
    if words.is_empty() {
        eprintln!("Warning: No words provided to remove stop words from.");
        return Ok(Vec::new());
    }

    let stop_words_content = fs::read_to_string("../stop_words.txt").map_err(|e| {
        eprintln!("Error reading stop words file: {}", e);
        e
    })?;
    
    let stop_words: Vec<String> = stop_words_content.split(',').map(|s| s.to_string()).collect();
    if stop_words.is_empty() {
        eprintln!("Warning: Stop words list is empty.");
    }

    let mut cleaned_words = Vec::new();
    for word in words {
        if word.len() > 1 && !stop_words.contains(&word) {
            cleaned_words.push(word);
        }
    }
    Ok(cleaned_words)
}

// Function to compute the frequencies of words
fn frequencies(words: Vec<String>) -> HashMap<String, usize> {
    if words.is_empty() {
        eprintln!("Warning: No words provided to compute frequencies.");
    }
    
    let mut word_freqs = HashMap::new();
    for word in words {
        *word_freqs.entry(word).or_insert(0) += 1;
    }
    word_freqs
}

// Function to sort the word frequencies by frequency in descending order
fn sort(word_freqs: HashMap<String, usize>) -> Vec<(String, usize)> {
    if word_freqs.is_empty() {
        eprintln!("Warning: No word frequencies to sort.");
    }

    let mut sorted_word_freqs: Vec<(String, usize)> = word_freqs.into_iter().collect();
    sorted_word_freqs.sort_by_key(|&(_, count)| Reverse(count));
    sorted_word_freqs
}

// Recursive function to print the word frequencies
fn print_all(word_freqs: &[(String, usize)], idx: usize) {
    if word_freqs.is_empty() {
        eprintln!("Warning: No word frequencies to print.");
        return;
    }

    if idx < word_freqs.len() {
        println!("{} - {}", word_freqs[idx].0, word_freqs[idx].1);
        print_all(word_freqs, idx + 1);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_process>", args[0]);
        std::process::exit(1);
    }

    let file_content = read_file(&args[1])?;
    if file_content.is_empty() {
        eprintln!("Error: The file '{}' is empty.", args[1]);
        std::process::exit(1);
    }

    let normalized_content = filter_chars_and_normalize(&file_content);
    let words = scan(&normalized_content);
    let filtered_words = remove_stop_words(words)?;
    let word_freqs = frequencies(filtered_words);
    let sorted_word_freqs = sort(word_freqs);

    // Print the top 25 words
    print_all(&sorted_word_freqs[0..sorted_word_freqs.len().min(25)], 0);

    Ok(())
}
