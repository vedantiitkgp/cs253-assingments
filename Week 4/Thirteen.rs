use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

// Utility functions
fn extract_words(data_storage: &mut HashMap<&str, String>, path_to_file: &str) {
    let mut file_content = String::new();
    if let Ok(mut file) = fs::File::open(path_to_file) {
        file.read_to_string(&mut file_content).unwrap();
    }
    let cleaned_content = file_content
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), " ");
    let words = cleaned_content.split_whitespace().map(String::from).collect::<Vec<_>>().join(" ");
    data_storage.insert("data", words);
}

fn load_stop_words(stop_words: &mut HashMap<&str, Vec<String>>) {
    if let Ok(content) = fs::read_to_string("../stop_words.txt") {
        let mut stop_words_vec: Vec<String> = content.split(',').map(String::from).collect();
        stop_words_vec.extend(('a'..='z').map(|c| c.to_string()));
        stop_words.insert("stop_words", stop_words_vec);
    }
}

fn increment_count(word_freqs: &mut HashMap<String, u32>, word: &str) {
    let count = word_freqs.entry(word.to_string()).or_insert(0);
    *count += 1;
}

fn is_stop_word(stop_words: &HashMap<&str, Vec<String>>, word: &str) -> bool {
    stop_words
        .get("stop_words")
        .map_or(false, |sw| sw.contains(&word.to_string()))
}

fn sorted(word_freqs: &HashMap<String, u32>) -> Vec<(String, u32)> {
    let mut freqs_vec: Vec<(String, u32)> = word_freqs.iter().map(|(k, &v)| (k.clone(), v)).collect();
    freqs_vec.sort_by(|a, b| b.1.cmp(&a.1));
    freqs_vec
}

// Main 'things'
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file name.");
        return;
    }
    let file_path = &args[1];

    // Data storage object equivalent
    let mut data_storage_obj: HashMap<&str, String> = HashMap::new();
    extract_words(&mut data_storage_obj, file_path);

    // Stop words object equivalent
    let mut stop_words_obj: HashMap<&str, Vec<String>> = HashMap::new();
    load_stop_words(&mut stop_words_obj);

    // Word frequencies object equivalent
    let mut word_freqs_obj: HashMap<String, u32> = HashMap::new();

    // Processing
    if let Some(data) = data_storage_obj.get("data") {
        let words: Vec<&str> = data.split_whitespace().collect();
        for word in words {
            if !is_stop_word(&stop_words_obj, word) {
                increment_count(&mut word_freqs_obj, word);
            }
        }
    }

    // Sorting and printing
    let sorted_freqs = sorted(&word_freqs_obj);
    for (word, count) in sorted_freqs.iter().take(25) {
        println!("{} - {}", word, count);
    }
}