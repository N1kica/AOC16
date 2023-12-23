use std::fs;

fn step(dir: i32, cords: (i32, i32), steps: i32) -> (i32, i32) {
    let (x, y) = cords;

    match dir {
        0 => (x, y + steps),
        1 => (x + steps, y),
        2 => (x, y - steps),
        3 => (x - steps, y),
        _ => cords,
    }
}

fn change_dir(val: i32, dir: &str) -> i32 {
    match dir {
        "R" => (((val + 1) % 4) + 4) % 4,
        _ => (((val - 1) % 4) + 4) % 4,
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut visited: Vec<(i32, i32)> = Vec::new();

    let contents: Vec<(&str, &str)> = file        
        .split(", ")
        .map(|instr| {
            let (dir, steps) = instr.split_at(1);
            (dir, steps)
        })
        .collect();

    let result = contents
        .iter()
        .fold((0, (0, 0)), |acc: (i32, (i32, i32)), x| {
            let value = change_dir(acc.0, x.0);
            let cords = step(value, acc.1, x.1.parse::<i32>().unwrap());

            for x in std::cmp::min(acc.1.0, cords.0)..=std::cmp::max(acc.1.0, cords.0) {
                for y in std::cmp::min(acc.1.1, cords.1)..=std::cmp::max(acc.1.1, cords.1) {
                    if !(x == acc.1.0 && y == acc.1.1) { 
                        if !visited.contains(&(x, y)) {
                            visited.push((x, y));
                        }
                        else {
                            println!("{:?}", (x, y));
                        }
                    }
                }
            }
            return (value, cords);
        });

    println!("{:?}", result);
}
