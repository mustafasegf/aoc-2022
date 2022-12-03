use std::{collections::HashSet, fs};

fn main() {
    let score = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .map(|(a, b)| {
            a.chars()
                .collect::<HashSet<_>>()
                .intersection(&b.chars().collect::<HashSet<_>>())
                .map(|c| *c)
                .filter(|v| v != &'\0')
                .collect::<Vec<_>>()
        })
        .map(|v| {
            v.to_owned()
                .into_iter()
                .map(|c| {
                    if c.is_uppercase() {
                        c as u32 - 'A' as u32 + 1 + 26
                    } else {
                        c as u32 - 'a' as u32 + 1
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    dbg!(score);
}
