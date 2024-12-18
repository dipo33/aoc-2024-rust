mod it_day;

use aoc_2024_rust::days::day_03::Day03;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
fn day03_part_1() {
    let expected: <Day03 as Day>::P1 = 156388521;
    test_part_1_uni::<Day03>(expected);
}

#[test]
fn day03_part_2() {
    let expected: <Day03 as Day>::P2 = 75920122;
    test_part_2_uni::<Day03>(expected);
}
