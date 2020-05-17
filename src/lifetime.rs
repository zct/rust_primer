
fn main() {
    min_lifetime();
    let s = String::from("hello world");
    let x = first_woard(&s);
    println!("{}",x);
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

fn first_woard(x :& String) ->& str{
    let x_bytes = x.as_bytes();
    for (i, &item) in x_bytes.iter().enumerate(){
        if item == b' '{
            return &x[0..i];
        }
    }
     &x[..]
}