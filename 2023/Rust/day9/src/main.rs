mod data;

const ANSWER_1: i64 = 1731106378;
const ANSWER_2: i64 = 1087;

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

fn compute_part1(data: &String) -> i64 {
    let mut sum = 0;

    let lines = data.lines().map(|line| line.split(" ").map(|x| {x.parse::<i64>().unwrap()}).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();

    for line in lines {
        let mut difference_vectors = vec![line];
        while difference_vectors.last().unwrap().iter().any(|x| {*x != 0}) {
            let mut new_vector: Vec<i64> = vec![];
            for i in 0..(difference_vectors.last().unwrap().len() - 1) {
                new_vector.push( difference_vectors.last().unwrap()[i + 1] - difference_vectors.last().unwrap()[i]);
            }
            difference_vectors.push(new_vector);
        }
        sum += difference_vectors.iter().map(|x| {x.last().unwrap()}).sum::<i64>();
    }
    sum
}

fn compute_part2(data: &String) -> i64 {
    let mut sum = 0;

    let lines = data.lines().map(|line| line.split(" ").map(|x| {x.parse::<i64>().unwrap()}).collect::<Vec<i64>>()).collect::<Vec<Vec<i64>>>();

    for line in lines {
        let mut difference_vectors = vec![line];
        while difference_vectors.last().unwrap().iter().any(|x| {*x != 0}) {
            let mut new_vector: Vec<i64> = vec![];
            for i in 0..(difference_vectors.last().unwrap().len() - 1) {
                new_vector.push( difference_vectors.last().unwrap()[i + 1] - difference_vectors.last().unwrap()[i]);
            }
            difference_vectors.push(new_vector);
        }
        for difference_vec in difference_vectors.iter().enumerate() {
            if difference_vec.0 % 2 == 0 {
                sum += difference_vec.1.first().unwrap()
            } else {
                sum -= difference_vec.1.first().unwrap() 
            }
        }
    }
    sum
}
