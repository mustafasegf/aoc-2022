use std::fs;

fn main() {
    let score = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| match s {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,

            _ => 0,
        })
        .sum::<i32>();

    dbg!(score);
}
