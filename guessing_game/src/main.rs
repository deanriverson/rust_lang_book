use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn read_guess() -> Result<u32, std::num::ParseIntError> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess.trim().parse();
}

/// Testing a doc comment
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let guess: u32 = match read_guess() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }
}
