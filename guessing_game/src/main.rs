use rand::Rng;
use std::cmp::Ordering;
use std::io; //cargo can generate docs for imported libraries with "cargo doc"

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //thread-local lazy

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //mut means mutable

        io::stdin()
            .read_line(&mut guess) //references should also set as mutable
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { //you can use same var name thanks to rust "shadowing"
            Ok(num) => num,
            Err(_) => continue, //_ means match all error types
        };

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
}
