use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}",args[1]);

    // for arguments in args.iter(){
    //     println!("{}",arguments);
    // }
}
