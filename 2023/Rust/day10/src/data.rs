use std::fs::File;
use std::io::prelude::*;

use std::fs::OpenOptions;
use std::io::Write;

pub fn get_data() -> String {
    let mut data: String = String::new();
    let mut file: File = File::open("data.txt").expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read file");

    let mut file = OpenOptions::new()
        .append(true) // Set the file to be opened in append mode
        .create(true) // Create the file if it doesn't exist
        .open("./modified_file.txt")
        .unwrap(); // Replace with your file path

    // This creates a far more readable
    // format for the puzzle, it requires
    // UTF-8 however.
    let edited = data
        .clone()
        .as_str()
        .replace("J", "┘")
        .replace("7", "┐")
        .replace("F", "┌")
        .replace("L", "└");

    write!(file, "{}", edited).unwrap(); // Write some text to the file

    edited
}

#[allow(dead_code)]
pub fn make_newlines_visible(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
