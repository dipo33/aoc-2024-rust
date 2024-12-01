mod it_day;

use aoc_2024_rust::days::day_01::Day01;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
fn day01_part_1() {
    let expected: <Day01 as Day>::P1 = 2086478;
    test_part_1_uni::<Day01>(expected);
}

#[test]
fn day01_part_2() {
    let expected: <Day01 as Day>::P2 = 24941624;
    test_part_2_uni::<Day01>(expected);
}
