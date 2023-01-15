fn main() {
    let mut z = 10;
    println!("Value of Z is: {}",z);

    z = 12;
    println!("Value of Z is: {}",z);

    shadowing();

}

fn shadowing() {
    let x = 2;

    let x = x + 1;

    {
        let x = x * 3;
        println!("the value of X is: {}", x);
    }

    println!("Value of x is : {}",x);

}
