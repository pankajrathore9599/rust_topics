
extern crate regex;

use regex::Regex;


fn main() {

    // ex1
    let re = Regex::new(r"\w{6}").unwrap();

    let text = "Panakj";

    println!("Found the correct: - {}", re.is_match(text));

    //ex2

    let re1 = Regex::new(r"(\w{6})").unwrap();

    match re1.captures(text) {
        Some(caps) => println!("Found match: {}",caps.get(0).unwrap().as_str()),
        None => println!("Could not found the match")
    }
}
