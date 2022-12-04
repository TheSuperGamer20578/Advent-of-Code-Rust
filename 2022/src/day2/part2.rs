//! https://adventofcode.com/2022/day/2#part2

#[cfg_attr(feature = "cargo-aoc", aoc(day2, part2))]
pub fn run(input: &[u8]) -> i64 {
    input.split(|byte| *byte == '\n' as u8)
        .map(|row| {
            let [them, _, outcome] = row else {return 0};
            let them = them - 'A' as u8;
            let outcome = *outcome as i64 - 'Y' as i64;
            let base = (outcome + 1) * 3;
            base as i64 + (them as i64 + outcome + 3) % 3 + 1
        })
        .sum()
}
