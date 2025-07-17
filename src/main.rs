use std::fs;

fn main() {
    let text = fs::read_to_string("002 logs.txt");
    match text {
        Ok(..) => println!("{:#?}", text),
        Err(error_message) => println!("{}", error_message),
    }
}
