use std::fs::File;

use std::io::prelude::*;

fn main() {

    // creating a file
    let mut file = File::create("output.txt").expect("Could not able to create.");

    // write to a file
    file.write_all(b"Welcome to freaky coder, and we will boom it \n").expect("Not able to write");

    // Append
    file.write_all(b"I am pankaj").expect("Not able to write");

}
