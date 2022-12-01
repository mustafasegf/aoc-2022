use itertools::Itertools;
use std::fs;

fn main() {
    let max = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<i32>().unwrap()).sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>();

    println!("{:?}", max);
}
