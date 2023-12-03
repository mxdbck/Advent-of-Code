use std::fs::File;
use std::io::prelude::*;

pub fn get_data() -> String {
    let mut data = String::new();
    let mut file = File::open("data.txt").expect("Unable to open file");
    file.read_to_string(&mut data).expect("Unable to read file");
    data
}

