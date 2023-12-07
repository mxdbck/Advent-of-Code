use std::{char, collections::HashSet};

mod data;

const NUMBERS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const ANSWER: u32 = 54649;

fn main() {
    let data = data::get_data();
    let result = compute_total(&data);
    println!("{}", result);
    // This is to make sure that any changes don't affect the answer
    assert_eq!(result, ANSWER)
}

fn compute_total(data: &String) -> u32{
    let lines = data.split("\r\n");
    let mut total = 0;
    for i in lines {
        total += parse_input(&i);
    }
    total
}

fn str_to_u32(s: &str) -> u32 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 10,
    }
}

fn parse_input(line: &str) -> u32 {
    let mut literal_number_set: HashSet<usize> = HashSet::new();
    for i in NUMBERS{
        for j in 0..line.len() {
            match line[j..].find(i) {
                Some(_) => (),
                None => continue,
            }
            literal_number_set.insert(line[j..].find(i).unwrap() + j);
        }
    }
    let mut first = "0".to_string();
    let mut last = "0".to_string();
    let chars = line.chars();
    let mut i = 0;
    while i < chars.clone().count() {
        let c_vec: Vec<char> = chars.clone().collect();
        let c = c_vec[i];
        if literal_number_set.contains(&i) {
            let mut len = 3;
            let mut number = str_to_u32(&line[i..i+len-1]);
            while number == 10 {
                len += 1;
                number = str_to_u32(&line[i..i+len-1]);
            }
            if first == "0" {
                first = number.to_string();
            } 
            last = number.to_string();
            i += 1;
            continue;
        }
        let parse_to_int = c.to_string().parse::<u32>();
        match parse_to_int {
            Ok(_) => {
                ()
            },
            Err(_) => {
                i += 1;
                continue;
            }
        }
        if first == "0" {
            first = c.to_string();
        }
        last = c.to_string();
        i += 1;
    }
    first.parse::<u32>().unwrap()*10 + last.parse::<u32>().unwrap()
}