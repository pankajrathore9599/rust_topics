// syntax1 
// let syn1 = [vl1,val2,val3];

//syntax 2
// let syn2 [datatype;5] = [val1,val2,val3,val4,val5];

//syntax3
// let syn3 [datatype;7] = [val1;7]



fn main() {
    println!("Arrays in rust");

    // let mut arr1 = [20,30,40,50];   // syntax 1

    // let arr2:[i32;4] = [10,30,50,70];  // syntax 2

    // let arr3:[i32;6] = [90;6];  // syntax 3



    // println!("{:?}", arr2);

    // println!("{:?}", arr3);

    // // mutable example

    // arr1[1] = 0;

    // println!("{:?}",arr1);


    // array as a parameter in function.
    let arr = [10,20,30];
    update(&mut arr);

    println!("Inside main function {:?}", arr);

}

fn update(mut arr:[i32;3]){
    for i in 0..3 {
        arr[i] = 0;
    }

    println!("Indnside update function {:?}",arr);
}
