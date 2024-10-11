use std::fs::File;
use std::io::{Read};
use std::env;

fn main() {
    let stop_words_path = "../stop_words.txt";
    let mut stop_words_file = File::open(stop_words_path).unwrap();
    let mut stop_words_content = String::new();
    stop_words_file.read_to_string(&mut stop_words_content).unwrap();
    let mut stop_words: Vec<String> = stop_words_content.split(',').map(|s| s.to_string()).collect();
    
    // Append single characters 'a' to 'z'
    stop_words.extend("abcdefghijklmnopqrstuvwxyz".chars().map(|c| c.to_string()));

    // Get the file to process from command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    let input_file_path = &args[1];
    let mut input_file = File::open(input_file_path).unwrap();
    let mut content = String::new();
    input_file.read_to_string(&mut content).unwrap();

    // Remove BOM (Byte Order Mark) if present Rust is Very Particular about this
    if content.starts_with('\u{feff}') {
        content = content[3..].to_string(); // Skip first 3 bytes
    }

    let mut word_freqs: Vec<(String, usize)> = vec![];
    let mut start_char = None;
    let mut i = 0;

    // Process the file character by character
    for c in content.chars() {
        if start_char.is_none() {
            if c.is_alphanumeric() {
                // Found the start of a word
                start_char = Some(i);
            }
        } else {
            if !c.is_alphanumeric() {
                // Found the end of a word
                let word_start = start_char.unwrap();
                let word_end = i;
                let word = content[word_start..word_end].to_lowercase();
                start_char = None;

                // Check if the word is not a stop word
                if !stop_words.contains(&word) {
                    let mut found = false;
                    let mut pair_index = 0;

                    // Check if word already exists in word_freqs
                    for (idx, pair) in word_freqs.iter_mut().enumerate() {
                        if pair.0 == word {
                            pair.1 += 1;
                            found = true;
                            pair_index = idx;
                            break;
                        }
                    }

                    // If word is not found, add it
                    if !found {
                        word_freqs.push((word.clone(), 1));
                        pair_index = word_freqs.len() - 1;
                    }

                    // Reorder if necessary (Bubble up the new frequency)
                    if word_freqs.len() > 1 {
                        for n in (0..pair_index).rev() {
                            if word_freqs[pair_index].1 > word_freqs[n].1 {
                                word_freqs.swap(n, pair_index);
                                pair_index = n;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }

    // Print top 25 word frequencies
    for pair in word_freqs.iter().take(25) {
        println!("{} - {}", pair.0, pair.1);
    }
}