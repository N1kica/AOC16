use std::{collections::HashMap, fs};
fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let contents: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let mut my_string = String::new();

    for i in 0..contents[0].len() {
        let mut char_count: HashMap<char, usize> = HashMap::new();
        for char_in_column in contents.iter().map(|row| row[i]) {
            *char_count.entry(char_in_column).or_insert(0) += 1;
        }

        let mut char_vec: Vec<_> = char_count.into_iter().collect();

        char_vec.sort_by(|a, b| a.1.cmp(&b.1));
        //char_vec.sort_by(|a, b| b.1.cmp(&a.1));
        let sorted_chars: Vec<_> = char_vec.into_iter().map(|(c, _)| c).collect();

        if let Some(first_char) = sorted_chars.first() {
            my_string.push(*first_char);
        }
    }

    println!("Result: {}", my_string);
}

// .sorted() ->
// ['a', 'a', 'a', 'b', 'c', 'd'] = [['a', 'b', 'c'], ['a', 'a', 'd']]
// .group_by() ->
// [('a', ['a', 'a', 'a']), ('b', ['b']), ('c', ['c']), ('d', ['d'])] =
// [[('a', ['a']), ('b', ['b']), ('c', ['c'])], [('a', ['a', 'a']), ('d', ['d'])]]
