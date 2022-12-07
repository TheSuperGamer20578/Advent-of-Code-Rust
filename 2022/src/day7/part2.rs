//! https://adventofcode.com/2022/day/7#part2
use std::collections::HashMap;

#[cfg_attr(feature = "cargo-aoc", aoc(day7, part2))]
pub fn run(input: &str) -> i64 {
    let mut dirs: HashMap<_, i64> = HashMap::new();
    let mut path = vec![];
    let mut free = 70000000;
    for command in input.trim()[2..].split("\n$ ") {
        let (command, output) = command.split_once('\n').unwrap_or((command, ""));
        match command.split_once(' ').unwrap_or((command, "")) {
            ("cd", "/") => {path.clear();},
            ("cd", "..") => {path.pop();},
            ("cd", dir) => {path.push(dir);},
            ("ls", _) => {
                for file in output.split('\n') {
                    let Ok(size) = file.split_once(' ').unwrap().0.parse::<i64>() else {continue;};
                    free -= size;
                    for i in 0..path.len() {
                        (*dirs.entry(path[..=i].join("/")).or_default()) += size;
                    }
                }
            }
            _ => unreachable!()
        }
    }
    let to_free = 30000000 - free;
    *dirs.values()
        .filter(|size| size >= &&to_free)
        .min().unwrap()
}