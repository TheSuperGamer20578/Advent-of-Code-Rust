//! https://adventofcode.com/2022/day/2

#[cfg_attr(feature = "cargo-aoc", aoc(day2, part1))]
pub fn run(input: &[u8]) -> i64 {
    input.split(|byte| *byte == '\n' as u8)
        .map(|row| {
            let [them, _, you] = row else {return 0};
            let them = them - 'A' as u8;
            let you = you - 'X' as u8;
            let base: i64 = (you + 1).into();
            if you == them {
                base + 3
            } else if you == (them + 1) % 3 {
                base + 6
            } else {
                base
            }
        })
        .sum()
}
