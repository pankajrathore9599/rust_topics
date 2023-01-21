fn main() {
    let number = 87;

    match number {
        1 => println!("It is one"),
        2 => println!("There is a no two"),
        // range
        3..=25 => println!("No is held between 3 to 25 somewhere"),
        _ => println!("None of them"),

    }
}
