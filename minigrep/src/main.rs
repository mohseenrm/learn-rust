use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    println!("Hello, world!");

    let query = &args[1];
    let filename = &args[2];

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
