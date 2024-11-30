mod it_day;

use aoc_2024_rust::days::day_06::Day06;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day06_part_1() {
    let expected: <Day06 as Day>::P1 = <Day06 as Day>::P1::default();
    test_part_1_uni::<Day06>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day06_part_2() {
    let expected: <Day06 as Day>::P2 = <Day06 as Day>::P2::default();
    test_part_2_uni::<Day06>(expected);
}
