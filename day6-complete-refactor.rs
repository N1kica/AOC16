use std::fs;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file"); 
    let contents: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let message = (0..contents[0].len()).map(|i| contents
        .iter()        
        .map(|row| row[i])
        .counts_by(|c| c)
    ).fold(Message::new(), |acc, group| {
        let max = group
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(ch, _)| ch)
            .unwrap();

        let min = group
            .iter()
            .min_by_key(|&(_, count)| count)
            .map(|(ch, _)| ch)
            .unwrap();

        Message {
            max: format!("{}{}", acc.max, max),
            min: format!("{}{}", acc.min, min),
        }
    });

    println!("Part 1: {}", message.max);
    println!("Part 2: {}", message.min);

}

struct Message {
    max: String,
    min: String,
}

impl Message {
    fn new() -> Self {
        Message {
            max: String::new(),
            min: String::new(),
        }
    }
}
