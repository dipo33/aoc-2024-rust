mod it_day;

use aoc_2024_rust::days::day_11::Day11;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day11 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day11 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day11_part_1() {
    test_part_1_uni::<Day11>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day11_part_2() {
    test_part_2_uni::<Day11>(EXPECTED_PART_2);
}
