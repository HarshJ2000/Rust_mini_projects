// Move the data from stack to heap

pub fn run() {
    let x = Box::new(50);
    println!("Value stored in heap using Box: {}", x);

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::*;

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list using Box: {:?}", list);
}
