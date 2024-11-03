use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::io::{self, Result};

// Pure function to filter non-alphanumeric characters and normalize to lowercase
fn filter_chars_and_normalize(text: &str) -> String {
    text.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { ' ' })
        .collect()
}

// Pure function to split the normalized text into words
fn scan(text: &str) -> Vec<String> {
    text.split_whitespace().map(|s| s.to_string()).collect()
}

// Pure function to remove stop words from a list of words
fn remove_stop_words(words: Vec<String>, stop_words: &[String]) -> Vec<String> {
    words
        .into_iter()
        .filter(|word| word.len() > 1 && !stop_words.contains(word))
        .collect()
}

// Pure function to compute the frequencies of words
fn frequencies(words: Vec<String>) -> HashMap<String, usize> {
    let mut word_freqs: HashMap<String, usize> = HashMap::new();
    for word in words {
        *word_freqs.entry(word).or_insert(0) += 1;
    }
    word_freqs
}

// Pure function to sort the word frequencies by frequency in descending order
fn sort(word_freqs: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_word_freqs: Vec<(String, usize)> = word_freqs.into_iter().collect();
    sorted_word_freqs.sort_by_key(|&(_, count)| Reverse(count));
    sorted_word_freqs
}

// Pure recursive function to convert word frequencies to strings for output
fn format_word_freqs(word_freqs: &[(String, usize)], idx: usize, result: &mut Vec<String>) {
    if idx < word_freqs.len() {
        result.push(format!("{} - {}", word_freqs[idx].0, word_freqs[idx].1));
        format_word_freqs(word_freqs, idx + 1, result);
    }
}

// IO function to read a file's content
fn read_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
}

// IO function to load stop words from a file
fn load_stop_words() -> Result<Vec<String>> {
    let content = fs::read_to_string("../stop_words.txt")?;
    Ok(content.split(',').map(|s| s.to_string()).collect())
}

// IO function to print word frequencies
fn print_all(formatted_freqs: Vec<String>) {
    for line in formatted_freqs {
        println!("{}", line);
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_process>", args[0]);
        std::process::exit(1);
    }

    // IO sequence: read file and stop words
    let file_content = read_file(&args[1])?;
    let stop_words = load_stop_words()?;

    // Pure computation sequence
    let normalized_content = filter_chars_and_normalize(&file_content);
    let words = scan(&normalized_content);
    let filtered_words = remove_stop_words(words, &stop_words);
    let word_freqs = frequencies(filtered_words);
    let sorted_word_freqs = sort(word_freqs);

    // Prepare output in a pure function
    let mut formatted_freqs = Vec::new();
    format_word_freqs(&sorted_word_freqs[0..sorted_word_freqs.len().min(25)], 0, &mut formatted_freqs);

    // IO sequence: print results
    print_all(formatted_freqs);

    Ok(())
}