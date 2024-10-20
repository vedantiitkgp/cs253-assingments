use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

struct Monad<T> {
    value: T,
}

impl<T> Monad<T> {
    // Wrap a value in the Monad
    fn new(value: T) -> Self {
        Monad { value }
    }

    // Bind a function to the value
    fn bind<F, U>(self, func: F) -> Monad<U>
    where
        F: FnOnce(T) -> U,
    {
        let new_value = func(self.value);
        Monad::new(new_value)
    }

    // Unwrap the value
    fn unwrap(self) -> T {
        self.value
    }
}

fn read_file(path_to_file: &str) -> Monad<String> {
    let mut file = fs::File::open(path_to_file).expect("Unable to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");
    Monad::new(data) // Return the data wrapped in a Monad
}

fn filter_chars(str_data: String) -> Monad<String> {
    let filtered: String = str_data.chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).collect();
    Monad::new(filtered) // Wrap the filtered data
}

fn normalize(str_data: String) -> Monad<Vec<String>> {
    let words: Vec<String> = str_data.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
    Monad::new(words) // Wrap the normalized words
}

fn remove_stop_words(word_list: Vec<String>) -> Monad<HashMap<String, usize>> {
    let stop_words = fs::read_to_string("../stop_words.txt")
        .expect("Unable to read stop words file")
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let filtered_words: Vec<String> = word_list.into_iter()
        .filter(|w| !stop_words.contains(w) && w.len() > 1)
        .collect();

    let mut wf: HashMap<String, usize> = HashMap::new();
    for word in filtered_words {
        *wf.entry(word).or_insert(0) += 1;
    }
    Monad::new(wf) // Wrap the word frequencies
}

fn sort(wf: HashMap<String, usize>) -> Monad<Vec<(String, usize)>> {
    let mut word_freqs: Vec<(String, usize)> = wf.into_iter().collect();
    word_freqs.sort_by(|a, b| b.1.cmp(&a.1));
    Monad::new(word_freqs) // Wrap the sorted frequencies
}

fn print_text(word_freqs: Vec<(String, usize)>) {
    for (w, c) in word_freqs.iter().take(25) {
        println!("{} - {}", w, c);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = read_file(&args[1])
        .bind(|data| filter_chars(data)) // Unwrap the data
        .bind(|filtered| normalize(filtered.unwrap())) // Unwrap filtered data
        .bind(|words| remove_stop_words(words.unwrap())) // Unwrap normalized words
        .bind(|wf| sort(wf.unwrap())) // Unwrap word frequencies
        .unwrap(); // Unwrap the final result
    print_text(result.unwrap()); // Unwrap result for printing
}
