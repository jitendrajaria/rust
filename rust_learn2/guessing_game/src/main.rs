use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number!!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        // println!("Secret number is {secret_number}");
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse::<u32>()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter the number!!");
                continue;
            }
        };

        // println!("Guess after parse {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You Won!!");
                break;
            }
        }
    }
}
