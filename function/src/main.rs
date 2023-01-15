fn main() {
    println!("Hello, world!");
    second_fun();

    // need to capture parameter
    third_fun(4);

    // passing for multiple value
    fouth_fun(89,'H');

    //expression example
    ex();

    // returning values in functions

    let x = return_values();
    println!("The value of X is: {}",x );

}

fn second_fun() {
    println!("function two");
}

// Passing signle parameters

fn third_fun(x: i32) {

    println!("the value of X is : {}",x);
}

// passing multple parameters

fn fouth_fun(value: i32, label: char) {
    println!("the pass value of measurement is {}  {}", value, label);
}

// expressions
fn ex(){
    let y = {
        let x = 9;
        x + 1
    };

    println!("The value of y is: {y}");
}


// returns value in functions

fn return_values() -> i32 {
    90 + 9
}