use std::cmp::Ordering;
use std::io;
// Rng is a "trait" of the rand crate. It defines methods that must be in scope
// for us to use the methods in rand.
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{guess} is too small!"),
            Ordering::Greater => println!("{guess} too big!"),
            Ordering::Equal => {
                println!("{guess} is right! You win!");
                break;
            }
        }
    }
}
