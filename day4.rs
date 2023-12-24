use std::{fs, collections::HashMap};

fn _is_valid_room(room: &str, checksum: &str) -> bool {
    let mut prev_count = 1000;
    let mut prev_char = 'a';

    for i in checksum.chars() {
        let letter_count = room.chars().filter(|&c| c == i).count();

        if letter_count < prev_count {
            prev_count = letter_count;
        } else if letter_count == prev_count {
            if i < prev_char {
                return false;
            } 
        } else {
            return false;
        }
        prev_char = i;
    }
    return true;
}

fn sort_room(input: &str) -> String {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    // Count occurrences of each character
    for c in input.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // Sort characters by their frequency in descending order
    let mut sorted_chars: Vec<_> = char_count.iter().filter(|(&c, _)| c >= 'a').collect();
    sorted_chars.sort_by(|a, b| {
        let count_cmp = b.1.cmp(a.1);
        if count_cmp == std::cmp::Ordering::Equal {
            a.0.cmp(b.0) // If counts are equal, sort alphabetically
        } else {
            count_cmp
        }
    });

    // Create the resulting string with characters sorted alphabetically
    sorted_chars.iter().map(|(c, _)| c.to_string()).collect()
}

fn _shift_room(text: &str, shift: i32) -> String {
    let mut result = String::new();
    for ch in text.chars() {
        if ch.is_ascii_alphabetic() {
            let base = if ch.is_ascii_lowercase() { b'a' } else { b'A' };
            let shifted = (((ch as i32 - base as i32) + shift + 26) % 26) as u8 + base;
            result.push(shifted as char);
        } else {
            result.push(ch);
        }
    }
    result
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file"); 

    let contents: Vec<Vec<&str>>  = file
        .lines()
        .map(|instr| {
            instr.split("[").map(|s| s.trim_end_matches(']')).collect()
        })
        .collect();

    let result = contents.iter().fold(0, |acc, room| {
        let sorted_room = sort_room(room[0]);

        if &sorted_room[..5] == room[1] {
            let last = room[0].split('-').last();
            acc + last.unwrap().parse::<i32>().unwrap()
        } else {
            acc
        }
    });

    println!("Result: {:?}", result);
}
