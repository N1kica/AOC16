use std::fs;

fn step(inst: char, cord: (usize, usize)) -> (usize, usize) {
    let keypad: Vec<Vec<char>> = vec![
        vec!['0', '0', '0', '0', '0', '0', '0'],
        vec!['0', '0', '0', '1', '0', '0', '0'],
        vec!['0', '0', '2', '3', '4', '0', '0'],
        vec!['0', '5', '6', '7', '8', '9', '0'],
        vec!['0', '0', 'A', 'B', 'C', '0', '0'],
        vec!['0', '0', '0', 'D', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0', '0', '0'],
    ];

    match inst {
        'U' => {
            if keypad[cord.0 - 1][cord.1] != '0' {
                (cord.0 - 1, cord.1)
            } else {
                cord
            }
        }
        'D' => {
            if keypad[cord.0 + 1][cord.1] != '0' {
                (cord.0 + 1, cord.1)
            } else {
                cord
            }
        }
        'L' => {
            if keypad[cord.0][cord.1 - 1] != '0' {
                (cord.0, cord.1 - 1)
            } else {
                cord
            }
        }
        'R' => {
            if keypad[cord.0][cord.1 + 1] != '0' {
                (cord.0, cord.1 + 1)
            } else {
                cord
            }
        }
        _ => cord,
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let keypad: Vec<Vec<char>> = vec![
        vec!['0', '0', '0', '0', '0', '0', '0'],
        vec!['0', '0', '0', '1', '0', '0', '0'],
        vec!['0', '0', '2', '3', '4', '0', '0'],
        vec!['0', '5', '6', '7', '8', '9', '0'],
        vec!['0', '0', 'A', 'B', 'C', '0', '0'],
        vec!['0', '0', '0', 'D', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0', '0', '0'],
    ];

    let contents: Vec<Vec<char>>  = file
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut code = String::new();
    let mut start = (1, 3);

    for line in contents {
        let number = line.iter().fold(start, |acc, inst| {
            step(*inst, acc)
        });
        code.push_str(&keypad[number.0][number.1].to_string());
        start = number;
    }

    println!("{}", code);
}
