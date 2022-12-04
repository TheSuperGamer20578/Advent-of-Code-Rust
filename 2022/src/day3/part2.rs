//! https://adventofcode.com/2022/day/3#part2

#[cfg_attr(feature = "cargo-aoc", aoc(day3, part2))]
pub fn run(input: &[u8]) -> i64 {
    input.split(|byte| *byte == '\n' as u8)
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|elves| {
            let [elf1, elf2, elf3] = elves else {unreachable!()};
            let priority = elf1.iter().filter(|item| elf2.contains(item))
                .filter(|item| elf3.contains(item))
                .next().unwrap();
            if *priority > 'Z' as u8 {
                (priority - 'a' as u8 + 1) as i64
            } else {
                (priority - 'A' as u8 + 27) as i64
            }
        })
        .sum()
}
