
fn main() {
    let s = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}