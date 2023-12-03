use std::collections::HashMap;

mod data;

const ANSWER_1: u32 = 2331;
const ANSWER_2: u32 = 71585;

fn main() {
    let data = data::get_data();
    let part1 = true;
    let result1 = compute_total(&data, part1);
    let result2 = compute_total(&data, !part1);
    println!("Result of Part 1: {}", result1);
    println!("Result of Part 2: {}", result2);
    // This is to make sure that any changes still result in the right answer
    assert_eq!(result1, ANSWER_1);
    assert_eq!(result2, ANSWER_2);
}

fn compute_total(data: &String, part1: bool) -> u32{
    let lines = data.split("\r\n");
    let mut total = 0;
    for i in lines {
        if part1 {
            let line_data = i.split(": ").collect::<Vec<&str>>();
            if is_possible(line_data[1]) {
                total += game_id(line_data[0]);
            }
        } else {
            let line_data = i.split(": ").collect::<Vec<&str>>();
            total += get_power(line_data[1]);
        }
    }
    total
}

fn bag_values(color: &str) -> u32{
    match color {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        &_ => u32::MIN,
    }
}

fn is_possible(data_string: &str) -> bool {
    let pulls = data_string.split("; ").collect::<Vec<&str>>();
    for i in pulls {
        let color_data = i.split(", ").collect::<Vec<&str>>();
        for j in color_data {
            let color_entry = j.split(" ").collect::<Vec<&str>>();
            if color_entry[0].parse::<u32>().unwrap() > bag_values(color_entry[1]) {
                return false;
            }
        }
    }
    true
}

fn get_power(data_string: &str) -> u32 {
    let pulls = data_string.split("; ").collect::<Vec<&str>>();
    let mut map: HashMap<&str, u32> = HashMap::new();
    for i in pulls {
        let color_data = i.split(", ").collect::<Vec<&str>>();
        for j in color_data {
            let color_entry = j.split(" ").collect::<Vec<&str>>();
            if map.contains_key(color_entry[1]) {
                if color_entry[0].parse::<u32>().unwrap() > *map.get(color_entry[1]).unwrap() {
                    map.insert(color_entry[1], color_entry[0].parse::<u32>().unwrap());
                }
            } else {
                map.insert(color_entry[1], color_entry[0].parse::<u32>().unwrap());
            }
        }
    }
    let mut total = 1;
    for i in map.iter() {
        total *= *i.1;
    }
    total
}

fn game_id(header: &str) -> u32 {
    return header.split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
}