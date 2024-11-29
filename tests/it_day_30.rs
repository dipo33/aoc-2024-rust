mod it_day;

use aoc_2024_rust::days::day_30::Day30;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day30 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day30 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day30_part_1() {
    test_part_1_uni::<Day30>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day30_part_2() {
    test_part_2_uni::<Day30>(EXPECTED_PART_2);
}
