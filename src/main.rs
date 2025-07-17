use std::fs;

fn main() {
    let text = fs::read_to_string("002 logs.txt").unwrap();
    println!("{:#?}", text);
}
