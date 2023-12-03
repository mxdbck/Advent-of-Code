use std::collections::{HashMap, HashSet};

mod data;

const ANSWER_1: u32 = 537732;
const ANSWER_2: u32 = 84883664;

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
    let lines = data.split("\r\n");
    let lines_vec = lines.collect::<Vec<&str>>();
    let mut total = 0;
    let map = find_allowed_locations(lines_vec.clone());
    for i in 0..lines_vec.len() {
        let chars = lines_vec[i].chars().collect::<Vec<char>>();
        let mut is_allowed = false;
        let mut num_as_str = String::new();
        for j in 0..chars.len() {
            if chars[j].is_ascii_digit() {
                if map.contains_key(&[i32::try_from(i).unwrap(), i32::try_from(j).unwrap()]) {
                    is_allowed = true;
                }
                num_as_str.push(chars[j]);
                continue;
            }
            if is_allowed {
                let current_value = num_as_str.parse::<u32>().unwrap();
                total += current_value;
            }
            is_allowed = false;
            num_as_str = String::new();
        }
        if is_allowed {
            let current_value = num_as_str.parse::<u32>().unwrap();
            total += current_value;
        }
    }
    total
}

fn find_allowed_locations(lines: Vec<&str>) -> HashMap<[i32; 2], bool> {
    let mut new_map: HashMap<[i32; 2], bool> = HashMap::new();
    for i in 0..lines.len() {
        let chars = lines[i].chars().collect::<Vec<char>>();
        for j in 0..chars.len() {
            if chars[j] != '.' && !chars[j].is_ascii_digit() {
                for k in 0..3 {
                    for l in 0..3 {
                        if k != 1 || l != 1 {
                            new_map.insert(
                                [
                                    i32::try_from(i + k).unwrap() - 1,
                                    i32::try_from(j + l).unwrap() - 1,
                                ],
                                true,
                            );
                        }
                    }
                }
            }
        }
    }
    new_map
}

fn compute_part2(data: &String) -> u32 {
    let char_matrix = data
        .split("\r\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let map = symbol_map(char_matrix.clone());
    let mut total = 0;
    map.iter().for_each(|(k, v)| -> () {
        if v.len() == 2 {
            let mut iter = v.iter();
            let first = get_val(&char_matrix, iter.next().unwrap());
            let second = get_val(&char_matrix, iter.next().unwrap());
            total += first * second;
        }
    });
    total
}

fn get_val(char_matrix: &Vec<Vec<char>>, coords: &[i32; 2]) -> u32 {
    let mut x = coords[0];
    let y = coords[1];
    let mut num_as_str = String::new();
    while x >= 0 && x < char_matrix.len() as i32 {
        if char_matrix[y as usize][x as usize].is_ascii_digit() {
            num_as_str.push(char_matrix[y as usize][x as usize]);
            x += 1;
            continue;
        }
        break;
    }
    num_as_str.parse::<u32>().unwrap()
}

fn symbol_map(char_matrix: Vec<Vec<char>>) -> HashMap<[i32; 2], HashSet<[i32; 2]>> {
    let mut map: HashMap<[i32; 2], HashSet<[i32; 2]>> = HashMap::new();
    for i in 0..char_matrix.len() {
        for j in 0..char_matrix[i].len() {
            if char_matrix[i][j] != '.' && !char_matrix[i][j].is_ascii_digit() {
                let mut set: HashSet<[i32; 2]> = HashSet::new();
                for k in 0..3 {
                    for l in 0..3 {
                        if k != 1 || l != 1 {
                            let x = (i32::try_from(j + l).unwrap() - 1)
                                .clamp(0, char_matrix[i].len() as i32);
                            let y = (i32::try_from(i + k).unwrap() - 1)
                                .clamp(0, char_matrix.len() as i32);
                            if char_matrix[y as usize][x as usize].is_ascii_alphanumeric() {
                                set.insert(id_num_coords(&char_matrix, x, y));
                            }
                        }
                    }
                }
                map.insert([i32::try_from(j).unwrap(), i32::try_from(i).unwrap()], set);
            }
        }
    }
    map
}

fn id_num_coords(char_matrix: &Vec<Vec<char>>, x: i32, y: i32) -> [i32; 2] {
    let mut x_pntr = x;
    while x_pntr >= 0 {
        if !char_matrix[y as usize][x_pntr as usize].is_ascii_digit() {
            return [x_pntr + 1, y];
        }
        if x_pntr == 0 {
            return [x_pntr, y];
        }
        x_pntr -= 1;
    }
    [0, 0]
}
