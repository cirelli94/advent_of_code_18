extern crate mylib;

use mylib::*;

const PUZZLE: &str = include_str!("../data/input.txt");

fn main() {
    println!("Calculated frequency is: {}", calculate_frequency(PUZZLE));

    match find_twice_frequency(PUZZLE) {
        Some(x) => println!("First frequency my device reaches twice: {}", x),
        None => println!("No frequency reached twice."),
    }
}
