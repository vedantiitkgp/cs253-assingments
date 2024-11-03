use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::io::{self, Result};

// Function to read the contents of a file and return it as a string
fn read_file(path: &str) -> Result<String> {
    if path.is_empty() {
        eprintln!("Error: Provided file path is empty.");
        return Ok(String::new()); // Return empty content for safety
    }
    fs::read_to_string(path).or_else(|e| {
        eprintln!("Error reading file '{}': {}", path, e);
        Ok(String::new()) // Default to empty string on failure
    })
}

// Function to filter non-alphanumeric characters and normalize to lowercase
fn filter_chars_and_normalize(text: &str) -> String {
    if text.is_empty() {
        eprintln!("Warning: Input text is empty.");
        return String::new();
    }
    text.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { ' ' })
        .collect()
}

// Function to split the normalized text into words
fn scan(text: &str) -> Vec<String> {
    if text.is_empty() {
        eprintln!("Warning: Normalized text is empty.");
        return Vec::new();
    }
    text.split_whitespace().map(|s| s.to_string()).collect()
}

// Function to remove stop words from a list of words
fn remove_stop_words(words: Vec<String>) -> Result<Vec<String>> {
    if words.is_empty() {
        eprintln!("Warning: Word list is empty, nothing to filter.");
        return Ok(words); // Return the empty or unchanged list
    }
    
    let stop_words_content = fs::read_to_string("../stop_words.txt").unwrap_or_else(|e| {
        eprintln!("Error reading stop words file: {}", e);
        String::new() // Default to empty if file can't be read
    });
    
    let stop_words: Vec<String> = stop_words_content.split(',').map(|s| s.to_string()).collect();
    let cleaned_words: Vec<String> = words.into_iter()
        .filter(|word| word.len() > 1 && !stop_words.contains(word))
        .collect();
    
    if cleaned_words.is_empty() {
        eprintln!("Warning: All words were filtered out or empty stop words file.");
    }
    Ok(cleaned_words)
}

// Function to compute the frequencies of words
fn frequencies(words: Vec<String>) -> HashMap<String, usize> {
    if words.is_empty() {
        eprintln!("Warning: No words provided for frequency computation.");
        return HashMap::new(); // Default to empty frequency map
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
        eprintln!("Warning: Word frequency map is empty, nothing to sort.");
        return Vec::new();
    }
    
    let mut sorted_word_freqs: Vec<(String, usize)> = word_freqs.into_iter().collect();
    sorted_word_freqs.sort_by_key(|&(_, count)| Reverse(count));
    sorted_word_freqs
}

// Recursive function to print the word frequencies
fn print_all(word_freqs: &[(String, usize)], idx: usize) {
    if idx >= word_freqs.len() {
        return;
    }
    
    if idx < 25 {
        println!("{} - {}", word_freqs[idx].0, word_freqs[idx].1);
    }
    print_all(word_freqs, idx + 1);
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_process>", args[0]);
        std::process::exit(1);
    }

    let file_content = read_file(&args[1])?;
    if file_content.is_empty() {
        eprintln!("Error: File content is empty or could not be read.");
        return Ok(());
    }

    let normalized_content = filter_chars_and_normalize(&file_content);
    if normalized_content.is_empty() {
        eprintln!("Error: Normalized content is empty.");
        return Ok(());
    }

    let words = scan(&normalized_content);
    if words.is_empty() {
        eprintln!("Error: No words found after scanning content.");
        return Ok(());
    }

    let filtered_words = remove_stop_words(words)?;
    if filtered_words.is_empty() {
        eprintln!("Error: No words left after removing stop words.");
        return Ok(());
    }

    let word_freqs = frequencies(filtered_words);
    if word_freqs.is_empty() {
        eprintln!("Error: No word frequencies to process.");
        return Ok(());
    }

    let sorted_word_freqs = sort(word_freqs);
    if sorted_word_freqs.is_empty() {
        eprintln!("Error: No sorted word frequencies to display.");
        return Ok(());
    }

    // Print the top 25 words
    print_all(&sorted_word_freqs, 0);

    Ok(())
}