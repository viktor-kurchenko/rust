use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Generating secret number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess and enter the number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
