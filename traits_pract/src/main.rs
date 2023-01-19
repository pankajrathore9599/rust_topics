
struct Person {
    name: String,
    age: u8
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("My name is {}, and i am of {}", self.name,self.age);
    }
}


fn main() {
    let my_id = Person{name:String::from("Pankaj Rathore"), age:25};

    println!("{}", my_id.to_string());
}
