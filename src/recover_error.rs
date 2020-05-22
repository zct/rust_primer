
use std::fs::File;

fn main() {
    read_file();
}

fn read_file() {
    let f = File::open("hello.txt").unwrap();
    // let f = match f .unwrap();
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };
}