extern crate mylib;

use mylib::*;

const INPUT: &str = include_str!("../data/input.txt");

pub fn main() {
    // part 1
    let result = INPUT
        .lines()
        .map(|l| count_letters(l))
        .fold((0usize, 0usize), |acc, (two, three)| {
            (acc.0 + two, acc.1 + three)
        });

    // println!("{:?}", result);
    println!("{}", result.0 * result.1);

    // part 2
    let lines: Vec<String> = INPUT.lines().map(|l| l.into()).collect();

    for (i, id) in lines.iter().enumerate() {
        for id2 in lines.iter().skip(i + 1) {
            if ids_prototypes(id, id2) {
                // println!("{}", id);
                // println!("{}", id2);

                println!(
                    "{}",
                    id.chars()
                        .zip(id2.chars())
                        .filter(|(a, b)| a == b)
                        .map(|(a, _)| a)
                        .collect::<String>()
                );
            }
        }
    }
}
