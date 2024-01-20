#[derive(Debug)]
enum List {
    Cons(usize,Box<List>),
    Nil
}

use List::{Cons,Nil};

fn main() {
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Cons(4,Box::new(Nil))))))));
    println!("{:?}",list);
}
