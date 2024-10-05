use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek, SeekFrom, Write};
use std::collections::HashMap;
use std::env;
use std::str;

// Memory cells: 1024 slots of memory, no labels, just index numbers
const MEMORY_SIZE: usize = 1024;
const STOP_WORDS_FILE: &str = "../stop_words.txt";
const WORD_FREQS_FILE: &str = "word_freqs";

fn touch_open(filename: &str) -> io::Result<File> {
    // Simulating a "touch" command to create a file if not exists, or reset it.
    if std::path::Path::new(filename).exists() {
        std::fs::remove_file(filename)?;
    }
    OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(filename)
}

fn main() -> io::Result<()> {
    // Memory simulation: All primary memory is stored in a fixed array
    let mut memory = vec!["".to_string(); MEMORY_SIZE];

    // Open the stop words file and load them into memory (cell 0 holds stop words)
    let stop_words_file = File::open(STOP_WORDS_FILE)?;
    let mut reader = BufReader::new(stop_words_file);
    let mut stop_words = String::new();
    reader.read_to_string(&mut stop_words)?;
    memory[0] = stop_words;

    // Cell 1 is reserved for processing lines from input
    memory[1] = "".to_string();

    // Cell 2 is for the current word being processed
    memory[2] = "".to_string();

    // Cell 3 is the word frequency count being processed from secondary memory
    memory[3] = "".to_string();

    // Cell 4 holds the file name passed from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_process>", args[0]);
        std::process::exit(1);
    }
    memory[4] = args[1].clone();

    // Cell 5 holds a flag for whether the word was found in secondary memory
    memory[5] = "false".to_string();

    // Cell 6 holds the current word to compare from the file
    memory[6] = "".to_string();

    // Cell 7 holds the current word frequency count
    memory[7] = "0".to_string();

    // Open the secondary memory for word frequencies
    let mut word_freqs_file = touch_open(WORD_FREQS_FILE)?;

    // Open the input file passed as argument
    let input_file = File::open(&memory[4])?;
    let reader = BufReader::new(input_file);

    // Process each line of the input file (memory[1] holds each line)
    for line in reader.lines() {
        memory[1] = line?;
        let line_length = memory[1].len();

        // Loop over characters in the line to extract words
        let mut word_start: Option<usize> = None;
        for (i, c) in memory[1].chars().enumerate() {
            if word_start.is_none() && c.is_alphanumeric() {
                // Word start found
                word_start = Some(i);
            } else if word_start.is_some() && !c.is_alphanumeric() {
                // Word end found, process it
                let word_end = i;
                memory[2] = memory[1][word_start.unwrap()..word_end].to_lowercase();
                word_start = None;

                // Filter words: check length and stop words
                if memory[2].len() >= 2 && !memory[0].split(',').any(|sw| sw == memory[2]) {
                    // Search for the word in secondary memory
                    word_freqs_file.seek(SeekFrom::Start(0))?;
                    memory[5] = "false".to_string();
                    while let Ok(line) = read_line_from_file(&mut word_freqs_file) {
                        if line.is_empty() {
                            break;
                        }
                        let parts: Vec<&str> = line.split(',').collect();
                        memory[6] = parts[0].to_string(); // word
                        memory[7] = parts[1].to_string(); // frequency

                        if memory[2] == memory[6] {
                            // Word found, increment count
                            let count: u32 = memory[7].parse().unwrap();
                            memory[7] = (count + 1).to_string();
                            memory[5] = "true".to_string();
                            break;
                        }
                    }

                    // Write the word and its count back to secondary memory
                    word_freqs_file.seek(SeekFrom::End(0))?;
                    if memory[5] == "false" {
                        writeln!(word_freqs_file, "{},{}", memory[2], 1)?;
                    } else {
                        // Replace the previous entry
                        word_freqs_file.seek(SeekFrom::End(-((memory[2].len() + 5) as i64)))?;
                        writeln!(word_freqs_file, "{},{}", memory[2], memory[7])?;
                    }
                }
            }
        }
    }

    // PART 2: Find the 25 most frequent words
    // Reset memory cells for sorting and processing
    for i in 0..25 {
        memory[i] = "".to_string();
    }

    let mut word_counts: Vec<(String, u32)> = vec![];

    word_freqs_file.seek(SeekFrom::Start(0))?;
    while let Ok(line) = read_line_from_file(&mut word_freqs_file) {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split(',').collect();
        let word = parts[0].to_string();
        let count: u32 = parts[1].parse().unwrap();
        word_counts.push((word, count));
    }

    word_counts.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by frequency in descending order

    // Print the top 25 words
    for (i, (word, count)) in word_counts.into_iter().take(25).enumerate() {
        println!("{} - {}", word, count);
        memory[i] = format!("{} - {}", word, count);
    }

    Ok(())
}

// Helper function to read a line from a file, for secondary memory handling
fn read_line_from_file(file: &mut File) -> io::Result<String> {
    let mut buf = Vec::new();
    let bytes_read = file.take(32).read_to_end(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf[..bytes_read]).to_string())
}
