use std::fs::File;
use std::io::prelude::*;

pub fn get_data() -> String {
    let mut data: String = String::new();
    let mut file: File = File::open("data.txt").expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read file");
    data
}

#[allow(unused)]
pub fn make_newlines_visible(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '\n' => "\\n".to_string(),
            '\r' => "\\r".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
