use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;
use std::rc::Rc;

// Utility function to read file content
fn read_file(path: &str) -> String {
    let mut content = String::new();
    if let Ok(mut file) = fs::File::open(path) {
        file.read_to_string(&mut content).unwrap_or_default();
    }
    content
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a file name.");
        return;
    }
    let file_path = &args[1];

    // Data storage object equivalent
    let mut data_storage_obj: HashMap<&str, Box<dyn Fn() -> String>> = HashMap::new();
    data_storage_obj.insert("data", Box::new(|| {
        let file_content = read_file(file_path);
        file_content
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric() && !c.is_whitespace(), " ")
    }));

    // Stop words object equivalent
    let mut stop_words_obj: HashMap<&str, Box<dyn Fn() -> Vec<String>>> = HashMap::new();
    stop_words_obj.insert("stop_words", Box::new(|| {
        let content = read_file("../stop_words.txt");
        let mut stop_words_vec: Vec<String> = content.split(',').map(String::from).collect();
        stop_words_vec.extend(('a'..='z').map(|c| c.to_string()));
        stop_words_vec
    }));

    // Word frequencies object equivalent
    let word_freqs = Rc::new(RefCell::new(HashMap::<String, u32>::new()));
    let mut word_freqs_obj: HashMap<&str, Box<dyn Fn(&str)>> = HashMap::new();
    let word_freqs_clone = Rc::clone(&word_freqs);

    word_freqs_obj.insert("increment", Box::new(move |word: &str| {
        let mut freqs = word_freqs_clone.borrow_mut();
        let count = freqs.entry(word.to_string()).or_insert(0);
        *count += 1;
    }));

    // Process words
    if let Some(data_closure) = data_storage_obj.get("data") {
        let data = data_closure();
        let words: Vec<&str> = data.split_whitespace().collect();

        if let Some(stop_words_closure) = stop_words_obj.get("stop_words") {
            let stop_words = stop_words_closure();

            for word in words {
                if !stop_words.contains(&word.to_string()) {
                    if let Some(increment_closure) = word_freqs_obj.get("increment") {
                        increment_closure(word);
                    }
                }
            }
        }
    }

    // Sort and display top 25 words
    let sorted_word_freqs: Vec<(String, u32)> = {
        let freqs = word_freqs.borrow();
        let mut freqs_vec: Vec<(String, u32)> = freqs.iter().map(|(k, &v)| (k.clone(), v)).collect();
        freqs_vec.sort_by(|a, b| b.1.cmp(&a.1));
        freqs_vec
    };

    for (word, count) in sorted_word_freqs.iter().take(25) {
        println!("{} - {}", word, count);
    }
}