use std::cmp::Ordering;

use aoc_common::day::Day;
use itertools::Itertools;

#[derive(Default)]
pub struct Day02 {}

#[derive(PartialEq)]
enum Trend {
    Increasing,
    Decreasing,
}

pub fn parse_input(input: &str) -> impl Iterator<Item = impl Iterator<Item = u32> + '_> + '_ {
    input.lines().map(|report| {
        report.split_whitespace().map(|level| {
            level.parse::<u32>().unwrap_or_else(|_| {
                panic!(
                    "each element in input should be a valid integer, found {}",
                    level
                )
            })
        })
    })
}

fn is_report_safe(report: impl Iterator<Item = u32>) -> bool {
    let mut trend = None;

    report.tuple_windows().all(|(a, b)| {
        let new_trend = match a.cmp(&b) {
            Ordering::Less => Trend::Increasing,
            Ordering::Greater => Trend::Decreasing,
            Ordering::Equal => return false,
        };

        match trend {
            None => trend = Some(new_trend),
            Some(ref trend) => {
                if *trend != new_trend {
                    return false;
                }
            }
        }

        (1..=3).contains(&(a.abs_diff(b)))
    })
}

fn is_report_safe_permissive(report: &[u32]) -> bool {
    (0..report.len())
        .map(|idx_to_omit| {
            report
                .iter()
                .enumerate()
                .filter_map(move |(idx, &val)| if idx_to_omit == idx { None } else { Some(val) })
        })
        .any(is_report_safe)
}

impl Day for Day02 {
    type P1 = u32;
    type P2 = u32;

    fn solve_part_1(&self, input: &str) -> Self::P1 {
        parse_input(input)
            .map(is_report_safe)
            .filter(|&x| x)
            .count() as u32
    }

    fn solve_part_2(&self, input: &str) -> Self::P2 {
        parse_input(input)
            .map(|report| is_report_safe_permissive(&report.collect::<Vec<_>>()))
            .filter(|&x| x)
            .count() as u32
    }

    fn get_day(&self) -> u8 {
        2
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let day = Day02::default();
        let part_1 = day.solve_part_1(&input);

        assert_eq!(part_1, 2)
    }

    #[test]
    fn test_example_part_2() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let day = Day02::default();
        let part_2 = day.solve_part_2(&input);

        assert_eq!(part_2, 4)
    }
}
