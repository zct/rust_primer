
fn main(){
    read_vector();
}

fn read_vector() {
    let a = vec![1,2,3,4];
    let third: &i32 = &a[2];
    
    match a.get(2) {
        Some(third) => println!("{}", third),
        None => println!("none"),
    }
}