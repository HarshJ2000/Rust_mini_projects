use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let arg1 = &args[1];
        let arg2 = &args[2];

        println!("{}", arg1);
        println!("{}", arg2);
    }
}
