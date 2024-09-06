use std::env;
use std::fs;
use std::io::ErrorKind;

// Функция для подсчета слов, строк и символов
pub fn analyze_text(text: &str) -> (usize, usize, usize) {
    let line_count = text.lines().count();
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    (word_count, line_count, char_count)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please provide a file path as an argument.");
        return;
    }

    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            let (word_count, line_count, char_count) = analyze_text(&contents);

            println!("Words: {}", word_count);
            println!("Lines: {}", line_count);
            println!("Characters: {}", char_count);
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => eprintln!("File not found: {}", file_path),
            ErrorKind::PermissionDenied => eprintln!("Permission denied to read the file: {}", file_path),
            _ => eprintln!("An error occurred: {:?}", error),
        },
    }
}
