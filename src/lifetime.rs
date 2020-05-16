
fn main() {
    min_lifetime();
}

fn min_lifetime() {
    let x = String::from("hello");
    let y = String::from("abc");
    let c = longest(x.as_str(), y.as_str());
    println!("{}",c);
}


fn longest<'a>(x :&'a str, y :&'a str) -> &'a str { //str是什么类型，跟string的区别
    if x.len() > y.len(){
        x
    } else {
        y
    }
}