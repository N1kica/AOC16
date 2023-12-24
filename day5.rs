


fn generate_password(door_id: &str) -> String {
    let mut password = String::from("--------");
    let mut index = 0;

    while password.len() < 9 {
        let input = format!("{}{}", door_id, index);
        let hash = md5::compute(input);

        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with("00000") {
            if let Some(sixth_char) = hash_str.chars().nth(5) {
                if let Some(sixth_digit) = sixth_char.to_digit(10) {
                    let sixth_as_usize = sixth_digit as usize;
                    if let Some(seventh_char) = hash_str.chars().nth(6) {
                        if sixth_as_usize < 8 {
                            password.replace_range(sixth_as_usize..sixth_as_usize + 1, &seventh_char.to_string());
                            println!("{:?}", password);
                        }
                    }
                }
            }
        }

        index += 1;
    }

    password
}

fn main() {
    let door_id = "ugkcyxxp";
    let password = generate_password(door_id);
    println!("The password for the door with ID {} is: {}", door_id, password);
}
