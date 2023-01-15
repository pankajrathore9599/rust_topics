fn main() {
    println!("Ownership tutorial");

    // let s = String::from("hello");   //32 kb
    // takes_ownership(s); // 32kb

    // let x = 5;
    // makes_copy(x);

    // println!("{}",x);

    let s1 = giver_ownership();  // 

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

// variable scope
// fn variable_owner() {
                        
//     let abc = "Hello";    // abc in the scope to use
// }

// // string scope
// fn string_owner() {
//     let ss = String::from("hello");

//     let mut ss = String::from("hello");
//     ss.push_str(", world");

//     println!("{}", ss);
// }

// // Memory allocation and transfer ownership

// fn var_trans(){
//     let x = 5;   // here x is the owner of value 5     // 32kb
//     let y = x;   // here we are passing the x ownership

//     println!("{}",y);

//     // string example
//     let s1 = String::from("Hello coder");    //64kb
//     let s2 = s1;

//     println!("{}",s2);
// }

// // clone example 
// // cloning the original value or memory of the original owner
// fn cl_own(){
//     let s4 = String::from("hello");
//     let s5 = s4.clone();

//     println!("s4 = {}, s5 = {} ", s4,s5);
// }

// Qwnership and functions

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer:i32) {
    println!("{}",some_integer);
}


// Return values and scope

fn giver_ownership() -> String {
    let s1 = String::from("My value");
    s1
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

