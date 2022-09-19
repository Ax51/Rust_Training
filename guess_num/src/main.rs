use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!(
        "Welcome to the Guess Number Game!
    
    The rules:    
    We came up with the number, you guess this number.
    "
    );

    let secret_num: u8 = rand::thread_rng().gen_range(0..=100);
    let mut last_low_guess = String::from("1");
    let mut last_high_guess = String::from("100");

    loop {
        println!("Input the number between {last_low_guess} and {last_high_guess}:");

        // println!("{secret_num}");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Incorrect input! {e}");
                continue;
            }
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => {
                last_low_guess = (guess + 1).to_string();
                if last_high_guess == last_low_guess {
                    println!("It was {secret_num}. You loose!");
                    break;
                }
                println!("{guess} is too small!");
            }
            Ordering::Greater => match guess.cmp(&100) {
                Ordering::Less => {
                    last_high_guess = (guess - 1).to_string();
                    if last_high_guess == last_low_guess {
                        println!("You loose!");
                        break;
                    }
                    println!("{guess} is too big!")
                }
                Ordering::Greater => println!("It can't be greather than 100! Try again"),
                Ordering::Equal => {
                    println!("{guess} is too big!")
                }
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
