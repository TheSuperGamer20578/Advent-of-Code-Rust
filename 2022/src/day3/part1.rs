//! https://adventofcode.com/2022/day/3

#[cfg_attr(feature = "cargo-aoc", aoc(day3, part1))]
pub fn run(input: &[u8]) -> i64 {
    input.split(|byte| *byte == '\n' as u8)
        .filter_map(|elf| {
            let half = elf.len() / 2;
            let priority = elf[..half].iter().filter(|item| elf[half..].contains(item))
                .next()?;
            if *priority > 'Z' as u8 {
                Some((priority - 'a' as u8 + 1) as i64)
            } else {
                Some((priority - 'A' as u8 + 27) as i64)
            }
        })
        .sum()
}
