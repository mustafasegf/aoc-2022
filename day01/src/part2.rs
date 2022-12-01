use itertools::Itertools;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    contents = contents.trim().to_string();

    let max = contents
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|s| s.parse::<i32>())
                .sum::<Result<i32, _>>()
                .unwrap()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>();

    println!("{:?}", max);
}
