mod it_day;

use aoc_2024_rust::days::day_23::Day23;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day23_part_1() {
    let expected: <Day23 as Day>::P1 = <Day23 as Day>::P1::default();
    test_part_1_uni::<Day23>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day23_part_2() {
    let expected: <Day23 as Day>::P2 = <Day23 as Day>::P2::default();
    test_part_2_uni::<Day23>(expected);
}
