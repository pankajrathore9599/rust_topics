use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess number GAME");

    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("Secret Number is {}",secret_number);

    loop {
        println!("Enter your number");

        let mut guess = String::new();
    
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read user input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("Your guessed number is {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small number: "),
            Ordering::Greater =>println!("Too Big numbber: "),
            Ordering::Equal => {
                println!("Great, You Won");
                break;
            }
        }
    }

}
