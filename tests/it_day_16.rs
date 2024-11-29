mod it_day;

use aoc_2024_rust::days::day_16::Day16;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day16 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day16 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day16_part_1() {
    test_part_1_uni::<Day16>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day16_part_2() {
    test_part_2_uni::<Day16>(EXPECTED_PART_2);
}
