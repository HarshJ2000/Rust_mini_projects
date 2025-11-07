// Lifetimes and borrowing 
// Program to take a string as an input and convert it into key/value pair which is of struct type

use std::{collections::HashMap, io};

fn main() {
    println!("Enter the string: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get inputs!!!!!!");

    let user_input = input.trim();

    println!("{:?}", parse_key_value_from_string(user_input));
}

fn parse_key_value_from_string(input: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for pair_str in input.split(';') {
        let parts: Vec<&str> = pair_str.split("=").collect();

        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            let value = parts[1].trim().to_string();
            map.insert(key, value);
        }
    }
    map
}
