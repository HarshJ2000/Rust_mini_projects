use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // skipping the first argument and just in case if <2 args given, providing instructions for the same
    if args.len() < 2 {
        println!("Usage: cargo run -- <number> [--double | --square]");
        std::process::exit(1);
    }

    // checking if the number argument provided is a valid integer or not? using pattern-matching on the args vector, .parse() helps to convert the number argument from string to integer of specified type annotation.
    let num: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a valid number!!!!!", &args[1]);
            std::process::exit(1);
        }
    };

    // checking if the operation flag is provided or not, if yes then which one. Here we used Option<T> for handling the inputs and we have used the .to_string() for the operation flags because the .contains will give us the &String of the args, so to match the types we have to make them referenced.
    let result = if args.contains(&"--double".to_string()) {
        Some(num * 2)
    } else if args.contains(&"--square".to_string()) {
        Some(num * num)
    } else {
        None
    };

    // pattern-matching the optionals for operation flags
    match result {
        Some(res) => println!("Result: {}", res),
        None => println!("No operation flag was provided!!!!!!"),
    }
}
