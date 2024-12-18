use aoc_common::day::{Day, Placeholder};

#[derive(Default)]
pub struct Day25 {}

impl Day for Day25 {
    type P1 = Placeholder;
    type P2 = Placeholder;

    fn solve_part_1(&self, _input: &str) -> Self::P1 {
        Placeholder
    }

    fn solve_part_2(&self, _input: &str) -> Self::P2 {
        Placeholder
    }

    fn get_day(&self) -> u8 {
        25
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[ignore = "not yet implemented"]
    fn test_example_part_1() {
        let input = r#""#;

        let day = Day25::default();
        let part_1 = day.solve_part_1(&input);

        assert_eq!(part_1, Placeholder)
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_example_part_2() {
        let input = r#""#;

        let day = Day25::default();
        let part_2 = day.solve_part_2(&input);

        assert_eq!(part_2, Placeholder)
    }
}
