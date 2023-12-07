mod data;

use std::collections::HashMap;

use regex::Regex;

const ANSWER_1: u32 = 0;
const ANSWER_2: u32 = 0;

fn main() {
    let data = data::get_data();
    let result1 = compute_part1(&data);
    println!("Result of Part 1: {}", result1);
    // let result2 = compute_part2(&data);
    // println!("Result of Part 2: {}", result2);
    // This is to make sure that any changes still result in the right answer
    // assert_eq!(result1, ANSWER_1);
    // assert_eq!(result2, ANSWER_2);
}

fn compute_part1(data: &String) -> u32 {
    let mut sum = 0;
    sum
}

fn compute_part2(data: &String) -> u32 {
    let mut sum = 0;
    sum
}

fn helper_function1() {

}

fn helper_function2() {

}
