
// Scalar & Compound Types

/*
Scalar Type
  - Integer 
                i8 = 8bit
                i16 = 16bit

                u8 - 8bit
                u16 - 16bit

                i8,i16,i32,i64,i128 - iarch
                u8,u16,u32,u64,u128 - uarch
  - Character  
                let a = 'z';
                let z: char ='Z'
                let emojis = '😊'
  - Number 
            let a = 43;
            let b 76 +32;
            let c = a + b
  - Floating 
                f32 - stores upto 32 bits 
                f64 - stores upto 64 bits of data

                let a = 45.90;
                let div = 456/89;

        
  - Boolean - True or False
            let a = True;

  - Decimal - let t = 7.90;


Compound Type
    - Tuple -   type of arrays
    - List - arrays
    - Dict
 */

fn main () {
    println!("Tuple example");

    let tup: (i32, f64, u8,) = (500,9.0,1);
    println!("{}",tup);

    println!("List example");
    
    let list1 = [1,2,3,4,5,7];
    let first_element = list1[4];
    println!("{}",first_element);
}