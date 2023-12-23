use std::fs;

fn is_right(num1: i32, num2: i32, num3: i32) -> bool {
    if num1 + num2 > num3 && num1 + num3 > num2 && num2 + num3 > num1 {
        return true
    } else {
        return false
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file"); 

    let contents: Vec<Vec<&str>>  = file
        .lines()
        .map(|instr| {
            instr.split_whitespace().collect()
        })
        .collect();

    let mut acc = 0;

    for x in (0..contents.len()).step_by(3) {
        for z in 0..contents[0].len() {
            if x + 3 <= contents.len() && is_right( contents[x][z].parse::<i32>().unwrap(), contents[x+1][z].parse::<i32>().unwrap(), contents[x+2][z].parse::<i32>().unwrap()) {
                acc += 1
            }
        }
    }

    println!("{:?}", acc);
}
