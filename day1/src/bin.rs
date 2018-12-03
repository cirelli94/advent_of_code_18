extern crate mylib;

use mylib::*;
use std::env;
use std::fs;

pub fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let input = fs::read_to_string("./data/input.txt").expect("Unable to read file");

    println!("Calculated frequency is: {}", calculate_frequency(&input));
}
