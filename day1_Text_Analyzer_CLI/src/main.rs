/* DAY 1 -> BUILDING A TEXT ANALYZER CLI THAT COUNTS WORD FREQUENCY */

use std::collections::HashMap;

fn main() {
    //------------------------------ METHOD 1 -----------------------------
    //---------------- This method takes the ownership which shouldn't be done -----------------
    let mut word_counts: HashMap<String, u32> = HashMap::new();
    let text = String::from("This that this why one too when too");

    for word in text.split_whitespace() {
        let word_string = word.to_lowercase().to_string();
        *word_counts.entry(word_string).or_insert(0) += 1;
    }

    for (word, count) in &word_counts {
        println!("{}:{}", word, count);
    }
}
