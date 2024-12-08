mod password;

use std::io;

fn main() {
    println!("How long do you want the password to be?");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    let password = password::generate_password(length);
    println!("Generated password: {}", password);
}
