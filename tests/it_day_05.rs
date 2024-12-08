mod it_day;

use aoc_2024_rust::days::day_05::Day05;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
fn day05_part_1() {
    let expected: <Day05 as Day>::P1 = 3608;
    test_part_1_uni::<Day05>(expected);
}

#[test]
fn day05_part_2() {
    let expected: <Day05 as Day>::P2 = 4922;
    test_part_2_uni::<Day05>(expected);
}
