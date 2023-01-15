fn main() {
    println!("If-Else Tutorial");

    first();

    second();

    third();
}

// if expression example

fn first(){
    let number = 3;

    if number < 5 {
        println!("Condition is True");
    }else {
        println!("Condition is False");
    }
}

// multiple if else example

fn second() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("This will not divisible by above numbers");
    }
}


// Using if in let statement

fn third(){

    let condition = true;
    let number = if condition {5} else {0};

    println!("The value of the number is {}", number);
}