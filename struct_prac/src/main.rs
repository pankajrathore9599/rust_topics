struct User {
    username: String,
    email: String,
    active: bool,
}


// fn main() {
//     println!("Struct Practice");

//     let user1 = User {
//         email: String::from("freakycoder@gmail.com"),
//         username: String::from("fraky8976"),
//         active: true
//     };

//     user1.email = String::from("abcd@gmail.com");
// }
// ---------------------------------------------------------------
struct Rectangle {
    width: u32,
    height: u32,
}

fn main(){

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle{
        width:89,
        height:78,
    };

    println!("The area of rectangle is {} square pixels", area(&rect1));

    println!("The area of rectangle is {} square pixels", area(&rect2));

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// --------------------------------------------------------------------------