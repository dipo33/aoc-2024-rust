mod it_day;

use aoc_2024_rust::days::day_02::Day02;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
fn day02_part_1() {
    let expected: <Day02 as Day>::P1 = 314;
    test_part_1_uni::<Day02>(expected);
}

#[test]
fn day02_part_2() {
    let expected: <Day02 as Day>::P2 = 373;
    test_part_2_uni::<Day02>(expected);
}
