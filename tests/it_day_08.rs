mod it_day;

use aoc_2024_rust::days::day_08::Day08;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day08_part_1() {
    let expected: <Day08 as Day>::P1 = <Day08 as Day>::P1::default();
    test_part_1_uni::<Day08>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day08_part_2() {
    let expected: <Day08 as Day>::P2 = <Day08 as Day>::P2::default();
    test_part_2_uni::<Day08>(expected);
}
