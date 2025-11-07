// Lifetimes and borrowing
// Program to take a string as an input and convert it into key/value pair which is of struct type

use std::{collections::HashMap, io};

fn main() {
    println!("Enter the string: ");

    let mut input = String::new();

    io::stdin() // Accepting input from user after code compilation
        .read_line(&mut input)
        .expect("Failed to get inputs!!!!!!"); // error handling

    let user_input = input.trim(); // removing new line tag

    println!("{:?}", parse_key_value_from_string(user_input));
}

fn parse_key_value_from_string<'a>(input: &'a str) -> HashMap<&'a str, &'a str> {
    // lifetimes using 'a
    let mut map = HashMap::new();

    for pair_str in input.split(';') {
        // splitting the string literal onto ';'
        let parts: Vec<&str> = pair_str.split("=").collect(); // iterating over parts around '=' and collecting them

        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            map.insert(key, value); // .insert method applied on hashmap
        }
    }
    map
}
