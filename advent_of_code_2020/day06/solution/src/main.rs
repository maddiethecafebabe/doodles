use std::{collections::HashSet, env, fs};

/* i wanted to do it in a single println today */

fn main() {
    println!(
        "Day6\n  Part1: {}\n  Part2: {}",
        fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))
            .unwrap()
            .split("\n\n")
            .map(|g| g
                .to_string()
                .replace("\n", "")
                .chars()
                .collect::<HashSet<char>>()
                .len())
            .sum::<usize>(),
        fs::read_to_string(env::args().nth(1).unwrap_or("input.txt".to_string()))
            .unwrap()
            .split("\n\n")
            .map(|g| {
                g.to_string()
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .map(|g| {
                "abcdefghijklmnopqrstuvwxyz"
                    .chars()
                    .filter(|&c| g.iter().filter(|p| p.contains(&c.to_string())).count() == g.len())
                    .count()
            })
            .sum::<usize>()
    );
}
