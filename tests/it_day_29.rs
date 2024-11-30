mod it_day;

use aoc_2024_rust::days::day_29::Day29;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day29_part_1() {
    let expected: <Day29 as Day>::P1 = <Day29 as Day>::P1::default();
    test_part_1_uni::<Day29>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day29_part_2() {
    let expected: <Day29 as Day>::P2 = <Day29 as Day>::P2::default();
    test_part_2_uni::<Day29>(expected);
}
