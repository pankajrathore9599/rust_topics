
use std::collections::HashMap;   // way to import package in rust


fn main() {
    let mut marks: HashMap<&str,i32> = HashMap::new();

    // add the values
    marks.insert("Rust", 89);
    marks.insert("Java", 67);
    marks.insert("Python", 99);
    marks.insert("Golang", 65);
    marks.insert("JS", 80);

    // Find the length
    println!("How many subjects have you studied: {}", marks.len());

    //get & match the values
    match marks.get("Java") {
        Some(mark) => println!("You got {} for java program", mark),
        None => println!("You did not study this subject")
    }

    // Remove value
    marks.remove("Python");
    println!("Subject remains in hashmap: - {}",marks.len());

    //Loop through hashmap
    for (subject,mark) in &marks {
        println!("For {} you got {}%", subject,mark);
    }

    // check the value
    println!("Did you study C++ {}", marks.contains_key("C++"));
}
