use std::fs::File;
use std::io::prelude::*;

fn main() {
    
    // open the file
    let mut file = File::open("./info.txt").expect("Coulkd not find the file on given path");

    // read the file
    let mut content = String::new();
    // priocess of reading the file
    file.read_to_string(&mut content).expect("Opps! Can't read the file");

    println!("File read succefully and Its Contains this information:- \n\n {}", content);
    
}
