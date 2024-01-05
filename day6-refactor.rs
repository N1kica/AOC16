use std::fs;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file"); 
    let contents: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();

    let groups = (0..contents[0].len()).map(|i| contents
        .iter()
        .map(|row| row[i])
        .sorted()
        .group_by(|&x| x)
    );

    let message = groups.fold(String::new(), |acc, group| {
        let max = group
            .into_iter()
            .map(|(key, group)| (key, group.count()))
            .max_by_key(|&(_, count)| count)
            .map(|(ch, _)| ch)
            .unwrap();

        format!("{}{}", acc, max)
    });

    println!("{}", message);
}
