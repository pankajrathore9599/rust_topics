fn main() {

    //     // syntax 1
    // let tup1:(data_type1,data2,data3) = (value1,value,2value3);

    // // syntax 2

    // let tup2 = (v1,v2,v3)

    let tuple1:(i32,f64,u8) = (-3,8.9,78);
    println!("{}, {}",tuple1.0, tuple1.1);

    // Destructing example

    let b:(i32,bool,f64) = (30,true,4.9);
    print(b);
}

fn print(x:(i32,bool,f64)) {
    println!("Inside print methods");
    let (age,is_male,CGPA) = x;
    println!("Age is {}, is Male {}, CGPA is {}", age,is_male,CGPA);

}
