fn main() {
    println!("Practising Loops");

    // continous loop
    // first();

    // second();

    // third();

    four();


}

// loop example
fn first() {
    loop{
        println!("Pankaj Rathore")
    }
}

// Return value from loop

fn second() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// Conditional while loop

fn third() {
    let mut number = 0;

    while number != 0 {
        println!("{}", number);

        number ==1;
    }

    println!("PANKAJ");
}

//

fn four() {
    let a = [10,20,30,40,50,60];

    let mut index = 0;

    while index < 7 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}