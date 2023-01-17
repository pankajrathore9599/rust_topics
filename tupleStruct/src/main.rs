
struct User(u8,u8,u8);

fn main() {
    println!("Tuple struct practice");

    let mut user = User(5,89,09);

    user.2 = 54;

    println!("the values are: {} ,{}, {}", user.0,user.1,user.2);



}
