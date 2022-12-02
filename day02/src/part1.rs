use std::fs;

fn main() {
    let score = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| match s {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,

            _ => 0,
        })
        .sum::<i32>();

    dbg!(score);
}
