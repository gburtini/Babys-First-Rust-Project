use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed: {} correctly!", guess);
                break;
            }
            Ordering::Greater | Ordering::Less => println!(
                "You guessed: {}, but the secret was {}",
                guess, secret_number
            ),
        };
    }
}
