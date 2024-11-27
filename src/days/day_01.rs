use aoc_common::day::{Day, Placeholder};

#[derive(Default)]
pub struct Day01 {}

impl Day for Day01 {
    type P1 = Placeholder;
    type P2 = Placeholder;

    fn solve_part_1(&self, _input: &[&str]) -> Self::P1 {
        Placeholder
    }

    fn solve_part_2(&self, _input: &[&str]) -> Self::P2 {
        Placeholder
    }

    fn get_day(&self) -> u8 {
        1
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[ignore]
    fn test_example_part_1() {
        todo!()
    }

    #[test]
    #[ignore]
    fn test_example_part2() {
        todo!()
    }
}
