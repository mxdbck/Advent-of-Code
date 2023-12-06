mod data;

use regex::Regex;

const ANSWER_1: u32 = 128700;
const ANSWER_2: u64 = 39594072;

fn main() {
    let data = data::get_data();
    let result1 = compute_part1(&data);
    println!("Result of Part 1: {}", result1);
    let result2 = compute_part2(&data);
    println!("Result of Part 2: {}", result2);
    // This is to make sure that any changes still result in the right answer
    assert_eq!(result1, ANSWER_1);
    assert_eq!(result2, ANSWER_2);
}

fn compute_part1(data: &String) -> u32 {
    let mut sum: u32 = 1;
    let regex = Regex::new(r"\d+").unwrap();
    let nums = regex.find_iter(data).map(|x| {x.as_str().parse::<u32>().unwrap()}).collect::<Vec<u32>>();
    for i in 0..(nums.len()/2) {
        sum *= (1..(nums[i])).map(|x| {(nums[i + nums.len()/2] < (x*(nums[i]-x))) as u32}).sum::<u32>();
    }
    sum
}

fn compute_part2(data: &String) -> u64 {
    let regex = Regex::new(r"\d+").unwrap();
    let nums = regex.find_iter(data.replace(" ", "").as_str()).map(|x| {x.as_str().parse::<u64>().unwrap()}).collect::<Vec<u64>>();
    (1..(nums[0])).map(|x| {(nums[1] < (x*(nums[0]-x))) as u64}).sum::<u64>()
}


