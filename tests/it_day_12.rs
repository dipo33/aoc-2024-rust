mod it_day;

use aoc_2024_rust::days::day_12::Day12;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day12 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day12 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day12_part_1() {
    test_part_1_uni::<Day12>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day12_part_2() {
    test_part_2_uni::<Day12>(EXPECTED_PART_2);
}
