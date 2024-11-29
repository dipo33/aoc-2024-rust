mod it_day;

use aoc_2024_rust::days::day_21::Day21;
use aoc_common::day::{Day, Placeholder};
use it_day::{test_part_1_uni, test_part_2_uni};

const EXPECTED_PART_1: <Day21 as Day>::P1 = Placeholder;
const EXPECTED_PART_2: <Day21 as Day>::P2 = Placeholder;

#[test]
#[ignore = "not yet implemented"]
fn day21_part_1() {
    test_part_1_uni::<Day21>(EXPECTED_PART_1);
}

#[test]
#[ignore = "not yet implemented"]
fn day21_part_2() {
    test_part_2_uni::<Day21>(EXPECTED_PART_2);
}
