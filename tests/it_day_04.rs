mod it_day;

use aoc_2024_rust::days::day_04::Day04;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
fn day04_part_1() {
    let expected: <Day04 as Day>::P1 = 2521;
    test_part_1_uni::<Day04>(expected);
}

#[test]
fn day04_part_2() {
    let expected: <Day04 as Day>::P2 = 1912;
    test_part_2_uni::<Day04>(expected);
}
