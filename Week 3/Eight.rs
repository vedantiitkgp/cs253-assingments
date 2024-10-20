use std::{env, fs, collections::HashMap};

fn main() -> std::io::Result<()> {
    let text = fs::read_to_string(env::args().nth(1).expect("Missing filename"))?;
    let stop_words_str = fs::read_to_string("../stop_words.txt")?;
    let stop_words: Vec<_> = stop_words_str.split(',').collect();
    let words = text.chars()
        .map(|c| if c.is_alphanumeric() { c.to_ascii_lowercase() } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .filter(|w| w.len() > 1 && !stop_words.contains(w))
        .fold(HashMap::new(), |mut map, word| {
            *map.entry(word.to_string()).or_insert(0) += 1;
            map
        });

    let mut word_vec: Vec<_> = words.into_iter().collect();
    quicksort(&mut word_vec);

    // Print top 25 words
    word_vec.iter().take(25).for_each(|(w, c)| println!("{} - {}", w, c));

    Ok(())
}

// Custom quicksort function for sorting word frequency
fn quicksort(arr: &mut [(String, i32)]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_idx = partition(arr);
    quicksort(&mut arr[0..pivot_idx]);
    quicksort(&mut arr[pivot_idx + 1..]);
}

// Partition function used by quicksort
fn partition(arr: &mut [(String, i32)]) -> usize {
    let pivot_idx = arr.len() - 1;
    let pivot = arr[pivot_idx].1;
    let mut i = 0;

    for j in 0..pivot_idx {
        if arr[j].1 > pivot { // Sort in reverse order
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot_idx);
    i
}