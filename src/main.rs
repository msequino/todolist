use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    loop {
            
        println!("Please input your guess.");
        guess.clear();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess.pop(); // remove the newline character

        match guess.cmp(&secret_number.to_string()) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}