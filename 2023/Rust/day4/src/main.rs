use std::collections::{HashMap, HashSet};

mod data;

const ANSWER_1: u32 = 28538;
const ANSWER_2: u32 = 9425061;

fn main() {
    let data = data::get_data();
    let result1 = compute_part1(&data);
    let result2 = compute_part2(&data);
    println!("Result of Part 1: {}", result1);
    println!("Result of Part 2: {}", result2);
    // This is to make sure that any changes still result in the right answer
    assert_eq!(result1, ANSWER_1);
    assert_eq!(result2, ANSWER_2);
}

fn compute_part1(data: &String) -> u32 {
    let mut total: f32 = 0.0;
    let lines = data.split("\r\n");
    lines.for_each(|x| -> () {
        let matches = calc_line(x);
        if matches == 0 {
            total += 0.0;
        } else{
            total += (2.0 as f32).powf((matches as f32) - 1.0);
        }
    });
    total.round() as u32
}

fn calc_line(line: &str) -> u32 {
    let data_line = line.split(": ").collect::<Vec<&str>>()[1];
    let line_vec = data_line.split(" | ").collect::<Vec<&str>>();
    let winning_vec = line_vec[0].split(" ").collect::<Vec<&str>>();
    let numbers_vec = line_vec[1].split(" ").collect::<Vec<&str>>();
    return numbers_vec.iter().map(|&x| {(winning_vec.contains(&x) && !x.is_empty()) as u32}).sum()
}

fn compute_part2(data: &String) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    let lines_vec = data.split("\r\n").collect::<Vec<&str>>();
    for i in 0..lines_vec.len() {
        increment_by_or_create_entry(&mut map, i as u32, 1);
        let to_repeat = *map.get(&(i as u32)).unwrap();
        for j in i..(calc_line(lines_vec[i]) as usize + i) {
            increment_by_or_create_entry(&mut map, (j as u32) + 1, to_repeat);
        }
    }
    return map.values().sum();
}

fn increment_by_or_create_entry(map: &mut HashMap<u32, u32>, key: u32, increment: u32) {
    if map.contains_key(&key) {
        map.insert(key, map.get(&key).unwrap() + increment);
    } else {
        map.insert(key, increment);
    }
}



// REALLY STUPID RECURSIVE APPROACH (WORKS BUT STAY AWAY)

// fn compute_part2(data: &String) -> f32 {
//     let mut total: f32 = 0.0;
//     let  mut map: HashMap<u32, u32> = HashMap::new();
//     let lines_vec = data.split("\r\n").collect::<Vec<&str>>();
//     let cards_to_process = (1..(lines_vec.len() as u32 + 1)).collect();
//     map_compute(&mut map, &mut data.split("\r\n").collect::<Vec<&str>>(), cards_to_process);
//     total += map.values().sum::<u32>() as f32;
//     total
// }

// fn map_compute(arg_map: &mut HashMap<u32, u32>, lines: &mut Vec<&str>, cards_to_process: Vec<u32>) {
//     for i in cards_to_process {
//         let line_num = lines[i as usize - 1].split(": ").collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>().last().unwrap().parse::<u32>().unwrap();
//         let to_add = arg_map.get_mut(&line_num);
//         match to_add {
//             Some(x) => {
//                 *x += 1;
//             },
//             None => {
//                 arg_map.insert(line_num, 1);
//             }
//         }
//         let matches = calc_line(lines[i as usize - 1]).round() as u32;
//         let mut new_cards_to_process: Vec<u32> = vec![];
//         for j in 1..matches+1 {
//             new_cards_to_process.push(line_num + j);
//         }
//         map_compute(arg_map, lines, new_cards_to_process);
//     }
// }