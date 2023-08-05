use std::env;
use std::fs;
fn main() {
    println!("Mini-Grep"); //Search string in a given file

    let args: Vec<String> = env::args().collect();
    println!("Query: {:?}", &args[1]);
    println!("Filename: {:?}", &args[2]);

    let content =
        fs::read_to_string(&args[2]).expect("Something went wrong while reading the file");
    println!("The File Content:{:?}", content);
}
