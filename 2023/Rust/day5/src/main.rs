mod data;

const ANSWER_1: i64 = 196167384;
const ANSWER_2: i64 = 125742456;

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
    let mut maps = data.split("\r\n\r\n");
    let seeds = maps
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap());

    // This creates an vector whose first index represents
    // each individual map, second index the line of the map
    // and third index the number on the line.
    let maps = maps
        .map(|x| {
            x.split("\r\n")
                .skip(1)
                .map(|x| {
                    x.split(" ")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect::<Vec<Vec<Vec<i64>>>>();

    let mut minimum = i64::MAX;
    for seed in seeds {
        let result = seed_location(seed, &maps);
        minimum = minimum.min(result[0]);
    }
    minimum
}

fn compute_part2(data: &String) -> i64 {
    let mut maps = data.split("\r\n\r\n");

    let seed_data = maps
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|x| (*x).parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut mininum = i64::MAX;

    // This creates an vector whose first index represents
    // each individual map, second index the line of the map
    // and third index the number on the line.
    let maps = maps
        .map(|x| {
            x.split("\r\n")
                .skip(1)
                .map(|x| {
                    x.split(" ")
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>()
        })
        .collect::<Vec<Vec<Vec<i64>>>>();

    for i in seed_data.iter().enumerate().step_by(2) {
        let mut result: [i64; 2];
        let mut skip = 0;
        for j in 0..seed_data[i.0 + 1] {
            if skip > 0 {
                skip -= 1;
                continue;
            } else {
                result = seed_location(*i.1 + j, &maps);
                mininum = mininum.min(result[0]);
                skip = result[1];
            }
        }
    }
    mininum
}

fn seed_location(seed: i64, maps: &Vec<Vec<Vec<i64>>>) -> [i64; 2] {
    let mut skip: i64 = i64::MAX;
    let mut var = seed;
    for i in maps {
        for j in i {
            if var >= j[1] && j[2] + j[1] > var {
                skip = skip.min(j[2] + j[1] - var - 1);
                var = (j[0] + var) - j[1];
                break;
            }
        }
    }
    [var, skip]
}


