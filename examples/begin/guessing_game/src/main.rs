use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // or gen_range(1..101)

    println!("Welcome to guess number game!");

    loop {
        println!("Enter the guess number: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read lines");

        /* When the user insert a non number the program crash
        let guess: u16 = guess.trim().parse().expect("Please type a number!");
        */

        let guess:u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please insert only numbers!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You rock!");
                break;
            }

        }
    }
}
