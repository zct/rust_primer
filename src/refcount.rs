
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Box::new(Cons(10, Box::new(Nil))))); //如何遍历这个List？

    let b = Cons(3, Rc::clone(&a));
    
    let c= Cons(3, Rc::clone(&a));
}