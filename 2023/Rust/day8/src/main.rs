mod data;

use std::collections::HashMap;

use regex::Regex;

const ANSWER_1: u128 = 21883;
const ANSWER_2: u128 = 12833235391111;

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

fn compute_part1(data: &String) -> u128 {
    let mut sum = 0;

    let mut line_iter = data.split("\r\n\r\n");
    let directions = line_iter
        .next()
        .unwrap()
        .split("")
        .filter(|x| *x != "")
        .map(|x| if x == "L" { 0 } else { 1 })
        .collect::<Vec<u128>>();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    let re = Regex::new(r"[A-Z]+").unwrap();

    line_iter.next().unwrap().lines().for_each(|x| {
        let mut iter = x.split(" = ");
        let key = iter.next().unwrap();
        let value = re
            .find_iter(iter.next().unwrap())
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();
        map.insert(key, value);
    });

    let start = "AAA";
    let end = "ZZZ";

    let mut current = start;
    let mut next_options;

    while current != end {
        next_options = map.get(current).unwrap();
        current = next_options[directions[sum % directions.len()] as usize];
        sum += 1;
    }
    sum as u128
}

#[derive(Debug)]
struct Encounter {
    z_version: String,
    steps: u128,
    steps_mod: u128,
}

fn compute_part2(data: &String) -> u128 {

    let mut line_iter = data.split("\r\n\r\n");

    // These start at 0 you ******* idiot
    let directions = line_iter
        .next()
        .unwrap()
        .split("")
        .filter(|x| *x != "")
        .map(|x| if x == "L" { 0 } else { 1 })
        .collect::<Vec<u128>>();

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    let re = Regex::new(r"[A-Z]+").unwrap();

    line_iter.next().unwrap().lines().for_each(|x| {
        let mut iter = x.split(" = ");
        let key = iter.next().unwrap();
        let value = re
            .find_iter(iter.next().unwrap())
            .map(|x| x.as_str())
            .collect::<Vec<&str>>();
        map.insert(key, value);
    });

    let end_is_a = Regex::new(r"[A-Z]*A").unwrap();
    let end_is_z = Regex::new(r"[A-Z]*Z").unwrap();

    let start = map
        .keys()
        .filter(|x| end_is_a.is_match(x))
        .map(|x| *x)
        .collect::<Vec<&str>>();

    let end = map
        .keys()
        .filter(|x| end_is_z.is_match(x))
        .map(|x| *x)
        .collect::<Vec<&str>>();

    let mut min_steps_to_z: HashMap<&str, Vec<Encounter>> = HashMap::new();

    let mut current = start.clone();

    for step in 0..1000000 {
        for j in current.iter_mut().enumerate() {
            *j.1 = map.get(j.1).unwrap()[directions[step % directions.len()] as usize];
            if end.contains(j.1) {
                if min_steps_to_z.contains_key(start[j.0]) {
                    min_steps_to_z.get_mut(start[j.0]).unwrap().push(Encounter {
                        z_version: j.1.to_string(),
                        steps: (step + 1) as u128,
                        steps_mod: ((step + 1) % directions.len()) as u128,
                    });
                } else {
                    min_steps_to_z.insert(
                        start[j.0],
                        vec![Encounter {
                            z_version: j.1.to_string(),
                            steps: (step + 1) as u128,
                            steps_mod: ((step + 1) % directions.len()) as u128,
                        }],
                    );
                }
            }
        }
    }

    // We observe that the steps to Z are periodic
    // And that the intial position is at equal
    // distance from a Z than every Z to the following Z
    // min_steps_to_z.keys().for_each(|x| {
    //     println!("{:?}", min_steps_to_z[x]);
    //     println!();
    // });

    // Since we start at the start of the cycle we can just
    // find the least common multiple of all cycle lengths
    // to find the first time all elements are at a Z
    lcm(&min_steps_to_z
        .values()
        .map(|x| x[0].steps)
        .collect::<Vec<u128>>())
        
}

fn lcm(nums: &Vec<u128>) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());
    a * b / gcd(a, b)
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
