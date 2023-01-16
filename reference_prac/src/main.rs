fn main() {
    println!("Reference Practice");

    // single reference
    let mut x = 15;
    let xy = &x;
    
    // multiple reference

    let z = &mut x;

    // change the value of the original owner

    *z += 1;

    println!("value of x is {}", x)
}
