use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

fn read_file(path_to_file: &str, func: fn(String)) {
    let mut file = fs::File::open(path_to_file).expect("Unable to open file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read file");
    func(data);  // Just pass the data to the next function (filter_chars)
}

fn filter_chars(str_data: String, func: fn(String)) {
    let filtered: String = str_data.chars().map(|c| if c.is_alphanumeric() { c } else { ' ' }).collect();
    func(filtered);  // Pass the filtered data to normalize
}

fn normalize(str_data: String, func: fn(Vec<String>)) {
    let words: Vec<String> = str_data.to_lowercase().split_whitespace().map(|s| s.to_string()).collect();
    func(words);  // Pass the words to remove_stop_words
}

fn remove_stop_words(word_list: Vec<String>, func: fn(HashMap<String, usize>)) {
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
    func(wf);  // Pass word frequencies to sort
}

fn sort(wf: HashMap<String, usize>, func: fn(Vec<(String, usize)>)) {
    let mut word_freqs: Vec<(String, usize)> = wf.into_iter().collect();
    word_freqs.sort_by(|a, b| b.1.cmp(&a.1));
    func(word_freqs);  // Pass sorted frequencies to print_text
}

fn print_text(word_freqs: Vec<(String, usize)>, func: fn()) {
    for (w, c) in word_freqs.iter().take(25) {
        println!("{} - {}", w, c);
    }
    func();
}

fn no_op() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    read_file(&args[1], |data| {
        filter_chars(data, |filtered| {
            normalize(filtered, |words| {
                remove_stop_words(words, |wf| {
                    sort(wf, |word_freqs| {
                        print_text(word_freqs, no_op);
                    });
                });
            });
        });
    });
}
