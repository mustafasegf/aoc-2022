use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let score = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|s| s.to_string())
                .collect_tuple::<(String, String, String)>()
                .unwrap()
        })
        .map(|(a, b, c)| {
            a.chars()
                .collect::<HashSet<_>>()
                .intersection(&b.chars().collect::<HashSet<_>>())
                .filter(|v| v != &&'\0')
                .map(|c| *c)
                .collect::<HashSet<_>>()
                .intersection(&c.chars().collect::<HashSet<_>>())
                .filter(|v| v != &&'\0')
                .map(|c| *c)
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
