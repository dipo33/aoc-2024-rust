mod it_day;

use aoc_2024_rust::days::day_14::Day14;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day14 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day14 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day14_part_1() {
    test_part_1_uni::<Day14>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day14_part_2() {
    test_part_2_uni::<Day14>(EXPECTED_PART_2);
}
