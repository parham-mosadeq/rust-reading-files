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

fn write_to_file(file_name: &str, content: String) {
    match fs::write(file_name, content) {
        Ok(..) => {
            println!(" successfully wrote to file");
        }

        Err(error_msg) => {
            println!("error_msg: {:#?}", error_msg);
        }
    }
}

fn main() {
    let text = fs::read_to_string("002 logs.txt");
    let err_msg: Vec<String>;
    let joined: String;
    match text {
        Ok(red_string) => {
            err_msg = read_errors(&red_string);
            joined = err_msg.join("\n");
            write_to_file("error.txt", joined);
        }
        Err(error_message) => {
            println!("{}", error_message);
        }
    }
}
