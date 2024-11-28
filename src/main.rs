use std::str::FromStr;

use aoc_common::day::Day;
use clap::Parser;
use days::{
    day_01::Day01, day_02::Day02, day_03::Day03, day_04::Day04, day_05::Day05, day_06::Day06,
    day_07::Day07, day_08::Day08, day_09::Day09, day_10::Day10, day_11::Day11, day_12::Day12,
    day_13::Day13, day_14::Day14, day_15::Day15, day_16::Day16, day_17::Day17, day_18::Day18,
    day_19::Day19, day_20::Day20, day_21::Day21, day_22::Day22, day_23::Day23, day_24::Day24,
    day_25::Day25, day_26::Day26, day_27::Day27, day_28::Day28, day_29::Day29, day_30::Day30,
};
use dotenvy::dotenv;

mod days;

#[derive(Debug, Clone)]
enum RunTarget {
    Day(u8),
    All,
}

impl FromStr for RunTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("all") {
            Ok(RunTarget::All)
        } else if let Ok(day) = s.parse::<u8>() {
            Ok(RunTarget::Day(day))
        } else {
            Err(format!("Invalid value for RunTarget: {}", s))
        }
    }
}

#[derive(Parser)]
struct Args {
    /// Specify the run target: either "all" or a day number (e.g., 1, 2, 3).
    #[arg(short, long)]
    run_target: RunTarget,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    run_target(args.run_target);
}

fn run_target(target: RunTarget) {
    match target {
        RunTarget::All => (1..=30).for_each(|day| run_target(RunTarget::Day(day))),
        RunTarget::Day(1) => Day01::run(),
        RunTarget::Day(2) => Day02::run(),
        RunTarget::Day(3) => Day03::run(),
        RunTarget::Day(4) => Day04::run(),
        RunTarget::Day(5) => Day05::run(),
        RunTarget::Day(6) => Day06::run(),
        RunTarget::Day(7) => Day07::run(),
        RunTarget::Day(8) => Day08::run(),
        RunTarget::Day(9) => Day09::run(),
        RunTarget::Day(10) => Day10::run(),
        RunTarget::Day(11) => Day11::run(),
        RunTarget::Day(12) => Day12::run(),
        RunTarget::Day(13) => Day13::run(),
        RunTarget::Day(14) => Day14::run(),
        RunTarget::Day(15) => Day15::run(),
        RunTarget::Day(16) => Day16::run(),
        RunTarget::Day(17) => Day17::run(),
        RunTarget::Day(18) => Day18::run(),
        RunTarget::Day(19) => Day19::run(),
        RunTarget::Day(20) => Day20::run(),
        RunTarget::Day(21) => Day21::run(),
        RunTarget::Day(22) => Day22::run(),
        RunTarget::Day(23) => Day23::run(),
        RunTarget::Day(24) => Day24::run(),
        RunTarget::Day(25) => Day25::run(),
        RunTarget::Day(26) => Day26::run(),
        RunTarget::Day(27) => Day27::run(),
        RunTarget::Day(28) => Day28::run(),
        RunTarget::Day(29) => Day29::run(),
        RunTarget::Day(30) => Day30::run(),
        RunTarget::Day(_) => eprintln!("Invalid day number, must be within 1 and 30"),
    }
}
