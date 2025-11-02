use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // skipping the first argument and just in case if <2 args given, providing instructions for the same
    if args.len() < 2 {
        println!("Usage: cargo run -- <number> [--double | --square]");
        std::process::exit(1);
    }
}
