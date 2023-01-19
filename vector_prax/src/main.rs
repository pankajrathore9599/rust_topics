fn main() {

    // syn1
    // let my_vector = Vec<i32> = Vec::new();

    let mut my_vec1 = vec![1,2,3,4,5];

    println!("{:?}", my_vec1);
    println!("{}", my_vec1[2]);

    // add the value in the vector
    my_vec1.push(89);
    println!("{:?}", my_vec1);

    // remove the value from vector
    my_vec1.remove(2);
    println!("{:?}", my_vec1);

    for number in my_vec1.iter() {
        println!("{}",number);
    }



}
