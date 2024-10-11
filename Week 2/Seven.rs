use std::{env, fs, collections::HashMap, cmp::Reverse};

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
    let mut sorted: Vec<_> = words.into_iter().collect();
    sorted.sort_unstable_by_key(|&(_, count)| Reverse(count));
    sorted.iter().take(25).for_each(|(w, c)| println!("{} - {}", w, c));
    Ok(())
}