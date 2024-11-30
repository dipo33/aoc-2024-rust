mod it_day;

use aoc_2024_rust::days::day_16::Day16;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day16_part_1() {
    let expected: <Day16 as Day>::P1 = <Day16 as Day>::P1::default();
    test_part_1_uni::<Day16>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day16_part_2() {
    let expected: <Day16 as Day>::P2 = <Day16 as Day>::P2::default();
    test_part_2_uni::<Day16>(expected);
}
