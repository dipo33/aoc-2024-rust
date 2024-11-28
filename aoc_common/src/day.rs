use std::fmt::{self, Display};

use crate::input_fetcher;

pub trait Day: Default {
    type P1: Display;
    type P2: Display;

    fn solve_part_1(&self, input: &[&str]) -> Self::P1;

    fn solve_part_2(&self, input: &[&str]) -> Self::P2;

    fn get_day(&self) -> u8;

    fn run() {
        run::<Self>();
    }
}

#[derive(Debug, PartialEq)]
pub struct Placeholder;

impl Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<PLACEHOLDER>")
    }
}

fn run<DAY>()
where
    DAY: Day,
{
    let instance = DAY::default();
    let day = instance.get_day();
    let input = input_fetcher::get_or_fetch_input(day)
        .expect(format!("Failed to fetch input for day {:02}", day).as_str());
    let input_lines: Vec<&str> = input.lines().collect();

    let part1_result = instance.solve_part_1(&input_lines);
    println!("Day {:02} - Part 1 Result: {}", day, part1_result);

    let part2_result = instance.solve_part_2(&input_lines);
    println!("Day {:02} - Part 2 Result: {}", day, part2_result);
}
