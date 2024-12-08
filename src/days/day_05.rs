use rayon::prelude::*;
use std::{cmp::Ordering, collections::HashMap};

use aoc_common::day::Day;

fn parse_rules(rules: &str) -> HashMap<u32, Vec<u32>> {
    let rules_iter = rules.lines().map(|rule| {
        let (fst, snd) = rule
            .split_once("|")
            .expect("each rule should contain two integers separated by |");
        (
            fst.parse::<u32>()
                .expect("each page should be a valid integer"),
            snd.parse::<u32>()
                .expect("each page should be a valid integer"),
        )
    });

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for (fst, snd) in rules_iter {
        rules.entry(fst).or_default().push(snd);
    }

    rules
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|page| {
                    page.parse::<u32>()
                        .expect("each page should be a valid integer")
                })
                .collect()
        })
        .collect()
}

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("input should contain two sections separated by an empty line");

    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    (rules, updates)
}

fn is_update_ordered(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    update
        .iter()
        .rev()
        .try_fold(Vec::new(), |mut forbidden_pages, &page| {
            if forbidden_pages.contains(&page) {
                None
            } else {
                if let Some(predecessors) = rules.get(&page) {
                    forbidden_pages.extend(predecessors);
                }
                Some(forbidden_pages)
            }
        })
        .is_some()
}

fn sort_update(update: &mut [u32], rules: &HashMap<u32, Vec<u32>>) {
    update.sort_by(|page_a, page_b| {
        if rules.get(page_a).map_or(false, |ps| ps.contains(page_b)) {
            Ordering::Greater
        } else if rules.get(page_b).map_or(false, |ps| ps.contains(page_a)) {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
}

fn get_middle_page(update: &[u32]) -> u32 {
    if update.len() % 2 == 0 {
        panic!("each update should have odd number of pages");
    }

    *update
        .get(update.len() / 2)
        .expect("each update should contain a middle page")
}

#[derive(Default)]
pub struct Day05 {}

impl Day for Day05 {
    type P1 = u32;
    type P2 = u32;

    fn solve_part_1(&self, input: &str) -> Self::P1 {
        let (rules, updates) = parse(input);

        updates
            .iter()
            .filter(|update| is_update_ordered(update, &rules))
            .map(|update| get_middle_page(update))
            .sum()
    }

    fn solve_part_2(&self, input: &str) -> Self::P2 {
        let (rules, updates) = parse(input);

        updates
            .into_par_iter()
            .filter(|update| !is_update_ordered(update, &rules))
            .map(|mut update| {
                sort_update(&mut update, &rules);
                update
            })
            .map(|update| get_middle_page(&update))
            .sum()
    }

    fn get_day(&self) -> u8 {
        5
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let day = Day05::default();
        let part_1 = day.solve_part_1(&input);

        assert_eq!(part_1, 143)
    }

    #[test]
    fn test_example_part_2() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let day = Day05::default();
        let part_2 = day.solve_part_2(&input);

        assert_eq!(part_2, 123)
    }
}
