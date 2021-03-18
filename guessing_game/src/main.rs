use rand::Rng;
use std::{cmp::Ordering, io::{self,ErrorKind,Error}};

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Result<Guess, Error> {
        if value < 1 || value > 100 {
            return Err(Error::new(ErrorKind::Other,"Value must be between 1 and 100"));
        }
        Ok(Guess { value })
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num){
                Ok(value) => value,
                Err(err) => {
                    println!("{}",err);
                    continue
                }
            },
            Err(_) => {
                println!("Input a valid value");
                continue
            }
        };
        println!("You guessed: {}", guess.value());
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
