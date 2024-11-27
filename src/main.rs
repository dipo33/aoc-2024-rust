use std::str::FromStr;

use aoc_common::day::Day;
use clap::Parser;
use days::day_01::Day01;
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
        // TODO: Don't forget to match all days here!!!
        RunTarget::Day(1) => Day01::run(),
        RunTarget::Day(_) => eprintln!("Invalid day number, must be within 1 and 30"),
    }
}
