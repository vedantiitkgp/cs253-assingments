use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};

// A trait for dispatching messages
trait MessageDispatch {
    fn dispatch(&mut self, message: Vec<&str>) -> Option<String>;
}

// DataStorageManager: Models the contents of the file
struct DataStorageManager {
    data: Vec<String>,
}

impl DataStorageManager {
    fn new() -> Self {
        DataStorageManager { data: Vec::new() }
    }

    fn init(&mut self, path_to_file: &str) {
        let content = fs::read_to_string(path_to_file).expect("Error reading file");
        self.data = DataStorageManager::filter_chars_and_normalize(content);
    }

    // Manually replace non-alphabetic characters with spaces and convert to lowercase
    fn filter_chars_and_normalize(content: String) -> Vec<String> {
        let mut filtered_content = String::new();

        for ch in content.chars() {
            if ch.is_alphabetic() {
                filtered_content.push(ch.to_ascii_lowercase());
            } else {
                filtered_content.push(' ');
            }
        }

        // Split by whitespace into words
        filtered_content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }

    fn words(&self) -> Vec<String> {
        self.data.clone()
    }
}

impl MessageDispatch for DataStorageManager {
    fn dispatch(&mut self, message: Vec<&str>) -> Option<String> {
        match message[0] {
            "init" => {
                self.init(message[1]);
                None
            }
            "words" => Some(self.words().join(" ")),
            _ => panic!("Message not understood {}", message[0]),
        }
    }
}

// StopWordManager: Models the stop word filter
struct StopWordManager {
    stop_words: Vec<String>,
}

impl StopWordManager {
    fn new() -> Self {
        StopWordManager {
            stop_words: Vec::new(),
        }
    }

    fn init(&mut self) {
        let file = fs::File::open("../stop_words.txt").expect("Cannot open stop_words.txt");
        let reader = io::BufReader::new(file);
        self.stop_words = reader
            .lines()
            .flat_map(|line| line.unwrap().split(',').map(String::from).collect::<Vec<String>>())
            .collect();
        // Add single-letter words a-z to stop words
        self.stop_words.extend((b'a'..=b'z').map(|c| (c as char).to_string()));
    }

    fn is_stop_word(&self, word: &str) -> bool {
        self.stop_words.contains(&word.to_string())
    }
}

impl MessageDispatch for StopWordManager {
    fn dispatch(&mut self, message: Vec<&str>) -> Option<String> {
        match message[0] {
            "init" => {
                self.init();
                None
            }
            "is_stop_word" => Some(self.is_stop_word(message[1]).to_string()),
            _ => panic!("Message not understood {}", message[0]),
        }
    }
}

// WordFrequencyManager: Keeps the word frequency data
struct WordFrequencyManager {
    word_freqs: HashMap<String, usize>,
}

impl WordFrequencyManager {
    fn new() -> Self {
        WordFrequencyManager {
            word_freqs: HashMap::new(),
        }
    }

    fn increment_count(&mut self, word: &str) {
        *self.word_freqs.entry(word.to_string()).or_insert(0) += 1;
    }

    fn sorted(&self) -> Vec<(String, usize)> {
        let mut word_freqs_vec: Vec<(String, usize)> = self.word_freqs.iter().map(|(w, c)| (w.clone(), *c)).collect();
        word_freqs_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending by count
        word_freqs_vec
    }
}

impl MessageDispatch for WordFrequencyManager {
    fn dispatch(&mut self, message: Vec<&str>) -> Option<String> {
        match message[0] {
            "increment_count" => {
                self.increment_count(message[1]);
                None
            }
            "sorted" => Some(
                self.sorted()
                    .iter()
                    .take(25)
                    .map(|(w, c)| format!("{} - {}", w, c))
                    .collect::<Vec<String>>()
                    .join("\n"),
            ),
            _ => panic!("Message not understood {}", message[0]),
        }
    }
}

// WordFrequencyController: The main controller
struct WordFrequencyController {
    storage_manager: DataStorageManager,
    stop_word_manager: StopWordManager,
    word_freq_manager: WordFrequencyManager,
}

impl WordFrequencyController {
    fn new() -> Self {
        WordFrequencyController {
            storage_manager: DataStorageManager::new(),
            stop_word_manager: StopWordManager::new(),
            word_freq_manager: WordFrequencyManager::new(),
        }
    }

    fn init(&mut self, path_to_file: &str) {
        self.storage_manager.dispatch(vec!["init", path_to_file]);
        self.stop_word_manager.dispatch(vec!["init"]);
    }

    fn run(&mut self) {
        let words = self
            .storage_manager
            .dispatch(vec!["words"])
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        for word in words {
            if self
                .stop_word_manager
                .dispatch(vec!["is_stop_word", &word])
                .unwrap()
                == "false"
            {
                self.word_freq_manager.dispatch(vec!["increment_count", &word]);
            }
        }

        println!(
            "{}",
            self.word_freq_manager.dispatch(vec!["sorted"]).unwrap()
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: word_frequency <path_to_file>");
        return;
    }

    let mut controller = WordFrequencyController::new();
    controller.init(&args[1]);
    controller.run();
}