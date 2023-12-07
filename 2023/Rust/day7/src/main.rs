mod data;

use std::collections::HashMap;

const ANSWER_1: u32 = 246912307;
const ANSWER_2: u32 = 246894760;

#[derive(Debug)]
enum LineData {
    Hand(String),
    Value(u32),
}

const ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const ORDER2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn main() {
    let data = data::get_data();
    let result1 = compute_part(&data, true);
    println!("Result of Part 1: {}", result1);
    let result2 = compute_part(&data, false);
    println!("Result of Part 2: {}", result2);
    // This is to make sure that any changes still result in the right answer
    assert_eq!(result1, ANSWER_1);
    assert_eq!(result2, ANSWER_2);
}

fn compute_part(data: &String, part1: bool) -> u32 {
    let mut map: HashMap<u32, Vec<[LineData; 2]>> = HashMap::new();
    let mut sum = 0;
    data.split("\r\n").for_each(|line| {
        let mut split = line.split(" ");
        let hand = split.next().unwrap();
        let value = split.next().unwrap().parse::<u32>().unwrap();
        map.entry(hand_type(hand, part1))
            .and_modify(|v| v.push([LineData::Hand(hand.to_string()), LineData::Value(value)]))
            .or_insert(vec![[
                LineData::Hand(hand.to_string()),
                LineData::Value(value),
            ]]);
    });
    let mut counter = 0;
    for i in 0..7 {
        if map.get(&i).is_none() {
            continue;
        }
        map.get_mut(&i)
            .unwrap()
            .sort_by(|a, b| hand_comparator(a, b, part1));
        map.get(&i).unwrap().iter().for_each(|x| {
            counter += 1;
            sum += match x[1] {
                LineData::Value(v) => v * counter,
                _ => panic!("Not a value"),
            };
        });
    }
    sum
}

fn hand_comparator(a: &[LineData; 2], b: &[LineData; 2], part1: bool) -> std::cmp::Ordering {
    let comp_vec: [char; 13];
    if part1 {
        comp_vec = ORDER;
    } else {
        comp_vec = ORDER2;
    }

    let a_hand = match &a[0] {
        LineData::Hand(s) => s,
        _ => panic!("Not a hand"),
    }
    .chars();
    let b_hand = match &b[0] {
        LineData::Hand(s) => s,
        _ => panic!("Not a hand"),
    }
    .chars();
    for (a, b) in a_hand.zip(b_hand) {
        match comp_vec
            .iter()
            .position(|&x| x == a)
            .cmp(&comp_vec.iter().position(|&x| x == b))
        {
            std::cmp::Ordering::Equal => continue,
            x => return x,
        }
    }
    panic!("Hands are equal");
}

fn hand_type(hand: &str, part1: bool) -> u32 {
    let mut map: HashMap<&str, u32> = HashMap::new();

    if part1 {
        for c in hand.split("") {
            if c == "" {
                continue;
            }
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    } else {
        let mut joker_count = 0;
        for c in hand.split("") {
            if c == "" {
                continue;
            }
            if c == "J" {
                joker_count += 1;
                continue;
            }
            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
        // We add the jokers to the most common card to create the best hand
        let max_k_v = map.iter().max_by(|a, b| a.1.cmp(b.1));
        if max_k_v.is_none() {
            map.insert("J", joker_count);
        } else {
            map.entry(max_k_v.unwrap().0)
                .and_modify(|v| *v += joker_count);
        }
    }

    // We can use the sum of the squares of the values to determine the hand type
    match map.values().map(|x| x.pow(2)).sum() {
        25 => 6, // "five of a kind",
        17 => 5, // "four of a kind",
        13 => 4, // "full house",
        11 => 3, // "three of a kind",
        9 => 2,  // "two pair",
        7 => 1,  // "one pair",
        _ => 0,  // "high card"
    }
}
