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

    let message = groups.fold(Message::new(), |acc, group| {
        let count: Vec<_> = group
            .into_iter()
            .map(|(key, group)| (key, group.count()))
            .collect();

        let max = count
            .iter()
            .max_by_key(|&(_, count)| count)
            .map(|(ch, _)| ch)
            .unwrap();
        
        let min = count
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

// trait CountByGroup {
//     fn count_by_group(self) -> BTreeMap<char, usize>; // -> std::collections::btree_map::IntoIter<char, usize>
// }

// impl<I> CountByGroup for I
// where
//     I: Iterator<Item = char>,
// {
//     fn count_by_group(self) -> BTreeMap<char, usize> {
//         let mut counts = BTreeMap::new();
// 
//         for c in self {
//             *counts.entry(c).or_insert(0) += 1;
//         }
// 
//         counts
//     }
// }
