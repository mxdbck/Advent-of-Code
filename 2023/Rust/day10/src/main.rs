// TODO : Make more general, the raycasting solution uses
// the fact that I know that S acts as a └, this might not
// be the case for other inputs, I should automatically
// determine what S acts like.

mod data;

const ANSWER_1: u32 = 6867;
const ANSWER_2: u32 = 595;

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

#[derive(Clone, Debug)]
struct Node {
    position: (i64, i64),
    directions: Vec<(i64, i64)>,
    entered_from: Option<(i64, i64)>,
}

fn compute_part1(data: &String) -> u32 {
    let char_matrix = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let s_index = find_in_matrix(&char_matrix, 'S').unwrap();
    let node_matrix = char_to_node(&char_matrix);
    let mut nodes_visited: Vec<Node> = Vec::new();
    nodes_visited.push(node_matrix[s_index.1 as usize][s_index.0 as usize].clone());
    nodes_visited.push(select_first_move_better(&node_matrix, s_index));
    while nodes_visited.last().unwrap().directions.len() != 0 {
        let last_node = nodes_visited.last().unwrap();
        let next_move = last_node
            .directions
            .iter()
            .filter(|x| last_node.entered_from.unwrap() != **x)
            .collect::<Vec<&(i64, i64)>>()[0];
        let next_index = (
            next_move.0 + last_node.position.0,
            next_move.1 + last_node.position.1,
        );
        let mut next_node = node_matrix[next_index.1 as usize][next_index.0 as usize].clone();
        next_node.entered_from = Some((next_move.0 * -1, next_move.1 * -1));
        nodes_visited.push(next_node);
    }
    return (nodes_visited.len() / 2) as u32;
}

fn compute_part2(data: &String) -> u32 {
    let mut sum = 0;
    let char_matrix = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let s_index = find_in_matrix(&char_matrix, 'S').unwrap();
    let node_matrix = char_to_node(&char_matrix);
    let mut nodes_visited: Vec<Node> = Vec::new();
    nodes_visited.push(node_matrix[s_index.1 as usize][s_index.0 as usize].clone());
    nodes_visited.push(select_first_move_better(&node_matrix, s_index));
    while nodes_visited.last().unwrap().directions.len() != 0 {
        let last_node = nodes_visited.last().unwrap();
        let next_move = last_node
            .directions
            .iter()
            .filter(|x| last_node.entered_from.unwrap() != **x)
            .collect::<Vec<&(i64, i64)>>()[0];
        let next_index = (
            next_move.0 + last_node.position.0,
            next_move.1 + last_node.position.1,
        );
        let mut next_node = node_matrix[next_index.1 as usize][next_index.0 as usize].clone();
        next_node.entered_from = Some((next_move.0 * -1, next_move.1 * -1));
        nodes_visited.push(next_node);
    }
    let all_visited = nodes_visited
        .iter()
        .map(|x| x.position)
        .collect::<Vec<(i64, i64)>>();
    let mut visited_per_row: Vec<Vec<i64>> = vec![vec![]; char_matrix.len()];
    for visited in all_visited {
        visited_per_row[visited.1 as usize].push(visited.0);
    }
    visited_per_row.iter_mut().for_each(|x| x.sort());

    for (row_index, row) in node_matrix.iter().enumerate() {
        for (col_index, _col) in row.iter().enumerate() {
            let node = nodes_visited
                .iter()
                .find(|x| x.position == (col_index as i64, row_index as i64));
            if node.is_none() {
                let mut edges = 0;
                for i in 1..(col_index.min(row_index) + 1) {
                    let new_x = col_index - i;
                    let new_y = row_index - i;
                    let diag_node = nodes_visited
                        .iter()
                        .find(|x| x.position == (new_x as i64, new_y as i64));
                    if diag_node.is_some() {
                        if ["└", "┐", "S"].contains(&char_matrix[new_y][new_x].to_string().as_str())
                        {
                            continue;
                        }
                        edges += 1;
                    }
                }
                if edges % 2 == 1 {
                    sum += 1;
                }
            }
        }
    }
    sum as u32
}

fn find_in_matrix(matrix: &Vec<Vec<char>>, target: char) -> Option<(i64, i64)> {
    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, &item) in row.iter().enumerate() {
            if item == target {
                return Some((col_index as i64, row_index as i64));
            }
        }
    }
    None
}

fn char_to_node(char_matrix: &Vec<Vec<char>>) -> Vec<Vec<Node>> {
    let mut node_matrix: Vec<Vec<Node>> = vec![
        vec![
            Node {
                position: (0, 0),
                directions: vec![],
                entered_from: None
            };
            char_matrix[0].len()
        ];
        char_matrix.len()
    ];
    for y in 0..char_matrix.len() {
        for x in 0..char_matrix[0].len() {
            node_matrix[y][x] = Node {
                position: (x as i64, y as i64),
                directions: char_to_directions(char_matrix[y][x]),
                entered_from: None,
            };
        }
    }
    node_matrix
}

fn char_to_directions(the_char: char) -> Vec<(i64, i64)> {
    match the_char {
        '┘' => vec![(-1, 0), (0, -1)],
        '┐' => vec![(-1, 0), (0, 1)],
        '┌' => vec![(1, 0), (0, 1)],
        '└' => vec![(1, 0), (0, -1)],
        '-' => vec![(-1, 0), (1, 0)],
        '|' => vec![(0, -1), (0, 1)],
        _ => vec![],
    }
}

fn select_first_move_better(node_matrix: &Vec<Vec<Node>>, current_position: (i64, i64)) -> Node {
    let to_test: Vec<(i64, i64)> = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for i in to_test {
        let x = (current_position.0 + i.0).clamp(0, node_matrix[0].len() as i64 - 1) as usize;
        let y = (current_position.1 + i.1).clamp(0, node_matrix.len() as i64 - 1) as usize;
        if node_matrix[y][x].directions.contains(&(i.0 * -1, i.1 * -1)) {
            let mut node = node_matrix[y][x].clone();
            node.entered_from = Some((i.0 * -1, i.1 * -1));
            return node;
        }
    }
    node_matrix[0][0].clone()
}