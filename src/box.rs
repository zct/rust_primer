
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>), //Cons是什么
    Nil,
}

struct MyBox<T>(T); //这一句声明的语法没看懂，不应该是大括号吗。只有一个变量的一种缩写？

impl<T> MyBox<T> {
    fn new(x :T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target =  T; //必须要这句声明，这里的作用是？
    fn deref(&self) -> &T {
        &self.0
    }
}

use crate::List::{Cons, Nil}; //使用结构体中的字段？
 
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    deref_pointer();
    deref_box_pointer();
    deref_mybox_pointer();
}

fn deref_pointer() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_box_pointer() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn deref_mybox_pointer() {
    let y = MyBox::new(5);

    assert_eq!(5, *y);
}