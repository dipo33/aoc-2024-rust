use aoc_common::{day::Day, input_fetcher};

pub fn test_part_1_uni<T: Day + Default>(expected: T::P1) {
    let day = T::default();
    let day_number = day.get_day();

    let input = input_fetcher::get_or_fetch_input(day_number)
        .expect(format!("Failed to fetch input for day {:02}", day_number).as_str());
    let input_lines: Vec<&str> = input.lines().collect();

    let part_1 = day.solve_part_1(&input_lines);
    assert_eq!(part_1, expected);
}

pub fn test_part_2_uni<T: Day + Default>(expected: T::P2) {
    let day = T::default();
    let day_number = day.get_day();

    let input = input_fetcher::get_or_fetch_input(day_number)
        .expect(format!("Failed to fetch input for day {:02}", day_number).as_str());
    let input_lines: Vec<&str> = input.lines().collect();

    let part_2 = day.solve_part_2(&input_lines);
    assert_eq!(part_2, expected);
}
