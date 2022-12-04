//! https://adventofcode.com/2022/day/1

#[cfg_attr(feature = "cargo-aoc", aoc(day1, part1))]
pub fn run(input: &str) -> i64 {
    input.split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|num| num.parse::<i64>().unwrap_or(0))
                .sum()
        })
        .max()
        .unwrap()
}
