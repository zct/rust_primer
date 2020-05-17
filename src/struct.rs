
fn main() {
    cal_area()
}

fn cal_area(){
    let x = Rectangle{
        width:2,
        height:3,
    };
    println!("{:?}", x);
}

#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}