mod it_day;

use aoc_2024_rust::days::day_21::Day21;
use aoc_common::day::Day;
use it_day::{test_part_1_uni, test_part_2_uni};

#[test]
#[ignore = "not yet implemented"]
fn day21_part_1() {
    let expected: <Day21 as Day>::P1 = <Day21 as Day>::P1::default();
    test_part_1_uni::<Day21>(expected);
}

#[test]
#[ignore = "not yet implemented"]
fn day21_part_2() {
    let expected: <Day21 as Day>::P2 = <Day21 as Day>::P2::default();
    test_part_2_uni::<Day21>(expected);
}
