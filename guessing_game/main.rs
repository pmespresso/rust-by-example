extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess what number I'm thinking of. Input your guess below ...");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
                    .expect("Failed to read line");

        println!("Your guess was {}", guess);

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too big!")
        };
    }
}
