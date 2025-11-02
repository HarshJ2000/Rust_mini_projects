use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // skipping the first argument and just in case if <2 args given, providing instructions for the same
    if args.len() < 2 {
        println!("Usage: cargo run -- <number> [--double | --square]");
        std::process::exit(1);
    }

    let num: i32 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: {} is not a valid number!!!!!", &args[1]);
            std::process::exit(1);
        }
    };

    let result = if args.contains(&"--double".to_string()) {
        Some(num * 2)
    } else if args.contains(&"--square".to_string()) {
        Some(num * num)
    } else {
        None
    };
}
