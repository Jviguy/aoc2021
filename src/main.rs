mod solutions;
use std::fs::File;
use std::env::args;
use std::io;
use std::io::prelude::*;
fn main() {
    let filename = args().nth(1).expect("Please provide a filename");
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    println!("{}", solutions::day4::part1(contents));
}
