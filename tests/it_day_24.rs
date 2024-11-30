mod it_day;

use aoc_2024_rust::days::day_24::Day24;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day24_part_1() {
    let expected: <Day24 as Day>::P1 = <Day24 as Day>::P1::default();
    test_part_1_uni::<Day24>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day24_part_2() {
    let expected: <Day24 as Day>::P2 = <Day24 as Day>::P2::default();
    test_part_2_uni::<Day24>(expected);
}
