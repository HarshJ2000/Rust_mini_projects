/* DAY 1 -> BUILDING A TEXT ANALYZER CLI THAT COUNTS WORD FREQUENCY */

use std::collections::HashMap;

fn main() {
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    let str = String::from("This that this why one too when too");

    for word in str.split_whitespace() {
        let word_string = word.to_lowercase().to_string();
        *word_counts.entry(word_string).or_insert(0) += 1;
    }

    println!("{:?}", word_counts);
}
