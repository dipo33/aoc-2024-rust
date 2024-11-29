mod it_day;

use aoc_2024_rust::days::day_17::Day17;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day17 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day17 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day17_part_1() {
    test_part_1_uni::<Day17>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day17_part_2() {
    test_part_2_uni::<Day17>(EXPECTED_PART_2);
}
