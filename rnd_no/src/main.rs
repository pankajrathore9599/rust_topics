extern crate rand;

use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..50); //min,max
    println!("Random Number is: {}", random_number);
}
