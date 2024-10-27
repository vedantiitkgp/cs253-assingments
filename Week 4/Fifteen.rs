use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

// A type alias for callback functions using Rc<RefCell<>> for shared ownership and mutability
type Callback<T> = Rc<RefCell<dyn Fn(T)>>;

struct WordFrequencyFramework {
    load_event_handlers: Vec<Callback<String>>,
    dowork_event_handlers: Vec<Callback<()>>,
    end_event_handlers: Vec<Callback<()>>,
}

impl WordFrequencyFramework {
    fn new() -> Self {
        WordFrequencyFramework {
            load_event_handlers: Vec::new(),
            dowork_event_handlers: Vec::new(),
            end_event_handlers: Vec::new(),
        }
    }

    // Register for load event
    fn register_for_load_event<F>(&mut self, handler: F)
    where
        F: 'static + Fn(String),
    {
        self.load_event_handlers.push(Rc::new(RefCell::new(handler)));
    }

    // Register for do work event
    fn register_for_dowork_event<F>(&mut self, handler: F)
    where
        F: 'static + Fn(()),
    {
        self.dowork_event_handlers.push(Rc::new(RefCell::new(handler)));
    }

    // Register for end event
    fn register_for_end_event<F>(&mut self, handler: F)
    where
        F: 'static + Fn(()),
    {
        self.end_event_handlers.push(Rc::new(RefCell::new(handler)));
    }

    // Run the framework
    fn run(&self, path_to_file: &str) {
        for handler in &self.load_event_handlers {
            handler.borrow()(path_to_file.to_string());
        }
        for handler in &self.dowork_event_handlers {
            handler.borrow()(());
        }
        for handler in &self.end_event_handlers {
            handler.borrow()(());
        }
    }
}

struct DataStorage {
    data: String,
    stop_word_filter: Rc<RefCell<StopWordFilter>>,
    word_event_handlers: Vec<Callback<String>>,
}

impl DataStorage {
    fn new(wfapp: &mut WordFrequencyFramework, stop_word_filter: Rc<RefCell<StopWordFilter>>) -> Rc<RefCell<Self>> {
        let storage = Rc::new(RefCell::new(DataStorage {
            data: String::new(),
            stop_word_filter,
            word_event_handlers: Vec::new(),
        }));
        
        let storage_clone = Rc::clone(&storage);
        wfapp.register_for_load_event(move |path_to_file| storage_clone.borrow_mut().load(path_to_file));
        
        let storage_clone = Rc::clone(&storage);
        wfapp.register_for_dowork_event(move |_| storage_clone.borrow().produce_words());
        
        storage
    }

    fn load(&mut self, path_to_file: String) {
        let contents = fs::read_to_string(path_to_file).expect("Failed to read file");
        let cleaned = contents
            .chars()
            .map(|c| if c.is_alphanumeric() { c.to_lowercase().to_string() } else { " ".to_string() })
            .collect::<String>();
        self.data = cleaned;
    }

    fn produce_words(&self) {
        for word in self.data.split_whitespace() {
            if !self.stop_word_filter.borrow().is_stop_word(word) {
                for handler in &self.word_event_handlers {
                    handler.borrow()(word.to_string());
                }
            }
        }
    }

    fn register_for_word_event<F>(&mut self, handler: F)
    where
        F: 'static + Fn(String),
    {
        self.word_event_handlers.push(Rc::new(RefCell::new(handler)));
    }
}

#[derive(Clone)]
struct StopWordFilter {
    stop_words: Vec<String>,
}

impl StopWordFilter {
    fn new(wfapp: &mut WordFrequencyFramework) -> Rc<RefCell<Self>> {
        let filter = Rc::new(RefCell::new(StopWordFilter {
            stop_words: Vec::new(),
        }));

        let filter_clone = Rc::clone(&filter);
        wfapp.register_for_load_event(move |_| filter_clone.borrow_mut().load());

        filter
    }

    fn load(&mut self) {
        let contents = fs::read_to_string("../stop_words.txt").expect("Failed to read stop_words.txt");
        self.stop_words = contents.split(',').map(|s| s.to_string()).collect();
        // Add single-letter words
        self.stop_words.extend(('a'..='z').map(|c| c.to_string()));
    }

    fn is_stop_word(&self, word: &str) -> bool {
        self.stop_words.contains(&word.to_string())
    }
}

struct WordFrequencyCounter {
    word_freqs: Rc<RefCell<HashMap<String, u32>>>,
}

impl WordFrequencyCounter {
    fn new(wfapp: &mut WordFrequencyFramework, data_storage: &Rc<RefCell<DataStorage>>) -> Rc<RefCell<Self>> {
        let counter = Rc::new(RefCell::new(WordFrequencyCounter {
            word_freqs: Rc::new(RefCell::new(HashMap::new())),
        }));
        
        let counter_clone = Rc::clone(&counter);
        data_storage.borrow_mut().register_for_word_event(move |word| counter_clone.borrow_mut().increment_count(word));
        
        let counter_clone = Rc::clone(&counter);
        wfapp.register_for_end_event(move |_| counter_clone.borrow().print_freqs());
        
        counter
    }

    fn increment_count(&mut self, word: String) {
        let mut freqs = self.word_freqs.borrow_mut();
        *freqs.entry(word).or_insert(0) += 1;
    }

    fn print_freqs(&self) {
        let freqs = self.word_freqs.borrow();
        let mut word_freqs: Vec<(&String, &u32)> = freqs.iter().collect();
        word_freqs.sort_by(|a, b| b.1.cmp(a.1));

        for (word, count) in word_freqs.iter().take(25) {
            println!("{} - {}", word, count);
        }
    }
}

struct WordsWithZCounter {
    z_word_count: Rc<RefCell<u32>>,
}

impl WordsWithZCounter {
    fn new(wfapp: &mut WordFrequencyFramework, data_storage: &Rc<RefCell<DataStorage>>) -> Rc<RefCell<Self>> {
        let counter = Rc::new(RefCell::new(WordsWithZCounter {
            z_word_count: Rc::new(RefCell::new(0)),
        }));
        
        let counter_clone = Rc::clone(&counter);
        data_storage.borrow_mut().register_for_word_event(move |word| {
            if word.contains('z') {
                *counter_clone.borrow_mut().z_word_count.borrow_mut() += 1;
            }
        });
        
        let counter_clone = Rc::clone(&counter);
        wfapp.register_for_end_event(move |_| {
            println!("Number of non-stop words containing 'z': {}", counter_clone.borrow().z_word_count.borrow());
        });
        
        counter
    }
}

fn main() {
    let mut wfapp = WordFrequencyFramework::new();
    let stop_word_filter = StopWordFilter::new(&mut wfapp);
    let data_storage = DataStorage::new(&mut wfapp, stop_word_filter);
    let _word_freq_counter = WordFrequencyCounter::new(&mut wfapp, &data_storage);
    let _words_with_z_counter = WordsWithZCounter::new(&mut wfapp, &data_storage);

    // Run the framework with the file path provided as command-line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        wfapp.run(&args[1]);
    } else {
        eprintln!("Please provide the path to the text file as an argument.");
    }
}