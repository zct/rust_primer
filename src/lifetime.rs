
fn main() {
    
}

fn min_lifetime() {
    let x = String::from("hello");
    let y = String::from("abc");
    let c = longest(x.as_str(), y.as_str());
    
}

fn longest<'a>(x :&'a str, y :&'a str) -> 'a str {
    if x.
    
}