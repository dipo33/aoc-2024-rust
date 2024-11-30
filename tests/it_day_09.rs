mod it_day;

use aoc_2024_rust::days::day_09::Day09;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day09_part_1() {
    let expected: <Day09 as Day>::P1 = <Day09 as Day>::P1::default();
    test_part_1_uni::<Day09>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day09_part_2() {
    let expected: <Day09 as Day>::P2 = <Day09 as Day>::P2::default();
    test_part_2_uni::<Day09>(expected);
}
