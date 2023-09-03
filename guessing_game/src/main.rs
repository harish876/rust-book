use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win");
                break;
            },
            Ordering::Less => println!("Your Guess is lower"),
            Ordering::Greater => println!("Your Guess is higher"),
        }
    }
}
