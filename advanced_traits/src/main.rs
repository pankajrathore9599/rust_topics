
struct Person {
    name: String,
    age: u8
}

trait Myhealth {
    //Run
    fn run(&self);

    // Not able to run
    fn not_run(&self) -> bool;
}

// trait salary
// trait family

impl Myhealth for Person {
    fn run(&self) {
        println!("Hello, My name is {}", self.name);
    }

    fn not_run(&self) -> bool {
        if self.age < 50 {
            return true;
        } return false;
    }
}

fn main() {
    let man1:Person = Person { name:String::from("Dev"), age: 45 };

    man1.run();

    println!("Can {} run {}", man1.name,man1.not_run());
}
