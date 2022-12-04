//! https://adventofcode.com/2022/day/1#part2

#[cfg_attr(feature = "cargo-aoc", aoc(day1, part2))]
pub fn run(input: &str) -> i64 {
    input.split("\n\n")
        .map(|elf| {
            elf.split('\n')
            .map(|num| num.parse::<i64>().unwrap_or(0))
            .sum()
        })
        .fold([0i64; 3], |mut acc, val: i64| {
            let (mini, min) = acc.iter()
            .enumerate()
            .reduce(|(acci, acc), (vali, val)| {
                if val < acc {
                    (vali, val)
                } else {
                    (acci, acc)
                }
            }).unwrap();
            if &val > min {
                acc[mini] = val;
            }
            acc
        })
        .iter()
        .sum()
}
