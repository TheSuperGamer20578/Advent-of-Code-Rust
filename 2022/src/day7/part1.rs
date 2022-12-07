//! https://adventofcode.com/2022/day/7
use std::collections::HashMap;

#[cfg_attr(feature = "cargo-aoc", aoc(day7, part1))]
pub fn run(input: &str) -> i64 {
    let mut dirs: HashMap<_, i64> = HashMap::new();
    let mut path = vec![];
    for command in input.trim()[2..].split("\n$ ") {
        let (command, output) = command.split_once('\n').unwrap_or((command, ""));
        match command.split_once(' ').unwrap_or((command, "")) {
            ("cd", "/") => {path.clear();},
            ("cd", "..") => {path.pop();},
            ("cd", dir) => {path.push(dir);},
            ("ls", _) => {
                for file in output.split('\n') {
                    let Ok(size) = file.split_once(' ').unwrap().0.parse::<i64>() else {continue;};
                    for i in 0..path.len() {
                        (*dirs.entry(path[..=i].join("/")).or_default()) += size;
                    }
                }
            }
            _ => unreachable!()
        }
    }
    dirs.values()
        .filter(|size| size <= &&100000)
        .sum()
}
