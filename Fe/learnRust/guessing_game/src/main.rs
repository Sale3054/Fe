use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    loop {
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1..100);

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        if guess.trim() == "q" {
            return;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(joe_momma) => joe_momma,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Greater, {}", secret_number),
            Ordering::Less => println!("Less, {}", secret_number),
            Ordering::Equal => {
                println!("Equals! {}", secret_number);
                break;
            }
        };
    }
}
