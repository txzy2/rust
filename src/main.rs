use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn sum(x: u32, y: u32) -> u32 {
    return x + y;
}

fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please type a valid number!"),
        }
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let guess = read_number("Please input your guess.");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let x = read_number("Paste x:");
    let y = read_number("Paste y:");

    println!("x + y = {}", sum(x, y));
}
