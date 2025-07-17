use std::fs;

fn read_errors(text: &str) -> Vec<String> {
    let splitted_text = text.split("\n");

    let mut error_lines = vec![];

    for line in splitted_text {
        if line.contains("ERROR") {
            error_lines.push(line.to_string());
        }
    }

    return error_lines;
}

fn main() {
    let text = fs::read_to_string("002 logs.txt");
    let err_msg: Vec<String>;
    match text {
        Ok(red_string) => {
            err_msg = read_errors(&red_string);
            println!("{:#?}", err_msg);
        }
        Err(error_message) => println!("{}", error_message),
    }
}
