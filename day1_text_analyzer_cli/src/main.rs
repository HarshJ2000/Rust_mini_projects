/* DAY 1 -> BUILDING A TEXT ANALYZER CLI THAT COUNTS WORD FREQUENCY */

use std::{collections::HashMap, fs};

fn main() {
    //------------------------------ First Step -----------------------------
    //* Knowing how to split words in a string & how to increment/decrement values in hashmaps using keys *//
    //
    // let mut word_counts: HashMap<String, u32> = HashMap::new();
    // let text = String::from("This that this why one too when too");

    // for word in text.split_whitespace() {
    //     let word_string = word.to_lowercase().to_string();
    //     *word_counts.entry(word_string).or_insert(0) += 1;
    // }

    // for (word, count) in &word_counts {
    //     println!("{}:{}", word, count);
    // }

    ///////////////////////////////

    //* Now implementing how to read from a file and using the contents to find out Total words also if they are Unique, longest, Shortest or not? *//
    let file_path = "./sample.txt";

    let contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file!!!!");

    let mut total_words: HashMap<String, u32> = HashMap::new();

    for word in contents.split_whitespace() {
        let content_to_string = word.to_lowercase();
        *total_words.entry(content_to_string).or_insert(0) += 1;
    }

    let unique_words: Vec<_> = total_words
        .iter()
        .filter(|&(_, &count)| count == 1)
        .collect();

    let mut longest_word: Option<&String> = None;
    let mut shortest_word: Option<&String> = None;

    let mut max_len: usize = 0;
    let mut min_len = usize::MAX;

    for (word, _) in &total_words {
        if word.len() > max_len {
            max_len = word.len();
            longest_word = Some(word);
        }

        if word.len() < min_len {
            min_len = word.len();
            shortest_word = Some(word);
        }
    }

    println!("Total Words: {:?}", total_words.len());
    println!("Unique Words: {:?}", unique_words.len());

    match longest_word {
        Some(word) => println!("Longest Word: {:?}", word),
        None => println!("None"),
    }

    match shortest_word {
        Some(word) => println!("Shortest Word: {:?}", word),
        None => println!("None"),
    }
}
