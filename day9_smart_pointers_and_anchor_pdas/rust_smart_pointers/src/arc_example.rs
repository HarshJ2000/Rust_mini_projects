// ARc Smart pointer -> Thread Safe Rc used for multi-threading

use std::sync::Arc;
use std::thread;

pub fn run() {
    let x = Arc::new(100);
    let mut handles = vec![];

    for _ in 0..5 {
        let x_clone = Arc::clone(&x);
        handles.push(thread::spawn(move || {
            println!("Value in thread: {}", x_clone);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}
