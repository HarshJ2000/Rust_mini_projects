// Shared Ownership for References
use std::rc::Rc;

pub fn run() {
    let a = Rc::new(10);
    println!("Refernce count initially: {}", Rc::strong_count(&a));

    let _b = Rc::clone(&a);
    println!("After cloning 1 time: {}", Rc::strong_count(&a));

    {
        let _c = Rc::clone(&a);
        println!("After cloning 2nd time: {}", Rc::strong_count(&a));
    } // reference count will decrease as _c will get out of scope

    println!("Reference count finally: {}", Rc::strong_count(&a));
}
