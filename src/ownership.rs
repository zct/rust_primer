fn string_mut() {
    
    let mut s = String::from("hello");
    s.push_str(" world");
    
    println!("{}", s);
}

fn string_copy() {
    let s1 = String::from("hello");  // let后面用不用mut有什么区别
    // let s2 = s1;
     let s2 = s1.clone();
    
    println!("{} {}", s1, s2);
}

fn string_return_value() {
    let s = String::from("hello");

    let s2 = takes_and_give_back(s);

    println!("{} ", s2);
}

fn takes_and_give_back(a_string: String) -> String{
    a_string
}

fn change(some_thing:&mut String) {
    some_thing.push_str(" test");
}

fn mutable_refrences() {
    // let mut s = String::from("hello");
    // s.push_str("world");
     //change(&s)
}

fn refrences_scope() {
    let mut s = String::from("hello");
    
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);  

    let  r3 = &mut s;
    r3.push_str(" socpe");
    println!("{} ", r3);
    println!("{} {}", r1, r2);  
}

fn main() {
//     string_mut();
//     string_copy();
//     string_return_value();
//    mutable_refrences();
   refrences_scope();
}
