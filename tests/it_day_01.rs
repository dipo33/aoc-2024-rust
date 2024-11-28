use aoc_2024_rust::days::day_01::Day01;
use aoc_common::day::{Day, Placeholder};
use aoc_common::input_fetcher;

const EXPECTED_PART_1: <Day01 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day01 as Day>::P2 = Placeholder;
const DAY_NUMBER: u8 = 1;

#[test]
#[ignore = "not yet implemented"]
fn test_part_1() {
    let input = input_fetcher::get_or_fetch_input(DAY_NUMBER)
        .expect(format!("Failed to fetch input for day {:02}", DAY_NUMBER).as_str());
    let input_lines: Vec<&str> = input.lines().collect();

    let day = Day01::default();
    let part_1 = day.solve_part_1(&input_lines);
    assert_eq!(part_1, EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn test_part_2() {
    let input = input_fetcher::get_or_fetch_input(DAY_NUMBER)
        .expect(format!("Failed to fetch input for day {:02}", DAY_NUMBER).as_str());
    let input_lines: Vec<&str> = input.lines().collect();

    let day = Day01::default();
    let part_2 = day.solve_part_2(&input_lines);
    assert_eq!(part_2, EXPECTED_PART_2);
}
