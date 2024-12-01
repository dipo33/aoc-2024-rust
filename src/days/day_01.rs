use std::collections::HashMap;

use aoc_common::day::Day;

#[derive(Default)]
pub struct Day01 {}

fn parse_line(line: &str) -> (u32, u32) {
    let (left, right) = line
        .split_once("   ")
        .expect("each line should contain two integers delimited by three spaces");

    (
        left.parse()
            .expect("left column should contain only valid integers"),
        right
            .parse()
            .expect("right column should contain only valid integers"),
    )
}

impl Day for Day01 {
    type P1 = u32;
    type P2 = u32;

    fn solve_part_1(&self, input: &str) -> Self::P1 {
        let (mut left_items, mut right_items): (Vec<_>, Vec<_>) =
            input.lines().map(parse_line).unzip();

        left_items.sort_unstable();
        right_items.sort_unstable();

        left_items
            .iter()
            .zip(right_items.iter())
            .map(|(left, &right)| left.abs_diff(right))
            .sum()
    }

    fn solve_part_2(&self, input: &str) -> Self::P2 {
        let (left_items, right_items): (Vec<_>, Vec<_>) = input.lines().map(parse_line).unzip();

        let mut frequencies = HashMap::new();
        for num in right_items {
            *frequencies.entry(num).or_insert(0) += 1;
        }

        left_items
            .iter()
            .map(|val| frequencies.get(val).copied().unwrap_or(0) * val)
            .sum()
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
    fn test_example_part_1() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        let day = Day01::default();
        let part_1 = day.solve_part_1(&input);

        assert_eq!(part_1, 11)
    }

    #[test]
    fn test_example_part_2() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        let day = Day01::default();
        let part_2 = day.solve_part_2(&input);

        assert_eq!(part_2, 31)
    }
}
