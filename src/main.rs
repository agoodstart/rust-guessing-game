use std::io;
use std::cmp::Ordering;
use std::thread::sleep;
use std::time::Duration;
use ansi_term::Style;
use ansi_term::Colour::{Green, Red, Black};
use rand::Rng;

fn main() {
    println!("{}", Style::new().bold().paint("Guess the number!"));

    let secret_number = rand::thread_rng().gen_range(1, 1001);
    println!("Please input your guess, or type quit to exit the program: \n");

    loop {
        
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "quit" {
            break;
        }
    
        let guess: u32 = match guess.trim().parse() {
            Ok(je_moeder) => je_moeder,
            Err(_) => {
                println!("Please, enter a number");
                continue;
            },
        };
    
        println!("You guessed {}", guess);

        sleep(Duration::new(1, 0));
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", Black.on(Red).paint(" Too small! Try again: ")),
            Ordering::Greater => println!("{}", Black.on(Red).paint(" Too big! Try again: ")),
            Ordering::Equal => {
                println!("{}", Black.on(Green).paint("You win!"));
                break;
            },
        }
    }

}
