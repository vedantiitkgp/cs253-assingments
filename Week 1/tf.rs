use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, BufRead, BufReader, Seek, SeekFrom, Write};
use std::env;
use std::str;

// Memory cells: 1024 slots of memory, no labels, just index numbers
const MEMORY_SIZE: usize = 1024;
const STOP_WORDS_FILE: &str = "../stop_words.txt";
const WORD_FREQS_FILE: &str = "../word_freqs.txt";

// Function to handle BOM (Byte Order Mark) at the beginning of the file
fn remove_bom(s: &str) -> &str {
    if s.starts_with("\u{feff}") {
        &s[3..]
    } else {
        s
    }
}

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

    // Open the secondary memory for word frequencies
    let mut word_freqs_file = touch_open(WORD_FREQS_FILE)?;

    // Read existing word frequencies into a HashMap for easy access
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    word_freqs_file.seek(SeekFrom::Start(0))?;
    while let Ok(line) = read_line_from_file(&mut word_freqs_file) {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split(',').collect();

        // Ensure the line has exactly 2 parts: word and frequency
        if parts.len() == 2 {
            let word = parts[0].to_string();
            if let Ok(count) = parts[1].trim().parse::<u32>() {
                word_counts.insert(word, count);
            } else {
                eprintln!("Failed to parse count for '{}'", parts[0]);
            }
        }
    }

    // Cell 4 holds the file name passed from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_to_process>", args[0]);
        std::process::exit(1);
    }
    memory[4] = args[1].clone();

    // Open the input file passed as argument
    let input_file = File::open(&memory[4])?;
    let reader = BufReader::new(input_file);

    // Process each line of the input file (memory[1] holds each line)
    for line in reader.lines() {
        let mut line = line?;

        // Remove BOM from the line if present
        line = remove_bom(&line).to_string();

        memory[1] = line.clone(); // Store the line in memory

        // Clone memory[1] to avoid conflicts
        let line_clone = memory[1].clone(); // Immutable clone of memory[1]

        // Loop over characters in the line to extract words
        let mut word_start: Option<usize> = None;

        for (i, c) in line_clone.chars().enumerate() {
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
                    // Increment the count in the HashMap
                    *word_counts.entry(memory[2].clone()).or_insert(0) += 1;
                }
            }
        }

        // Handle the case where the last word in a line is followed by the end of the line
        if let Some(start) = word_start {
            memory[2] = memory[1][start..].to_lowercase();
            if memory[2].len() >= 2 && !memory[0].split(',').any(|sw| sw == memory[2]) {
                *word_counts.entry(memory[2].clone()).or_insert(0) += 1;
            }
        }
    }

    // Write the updated counts back to the word frequency file
    word_freqs_file.set_len(0)?; // Clear the file before writing
    for (word, count) in &word_counts {
        writeln!(word_freqs_file, "{},{}", word, count)?;
    }

    // Print the top 25 words
    let mut sorted_counts: Vec<(String, u32)> = word_counts.iter().map(|(word, count)| (word.clone(), *count)).collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by frequency in descending order

    for (i, (word, count)) in sorted_counts.into_iter().take(25).enumerate() {
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