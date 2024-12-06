use std::ops::{Add, AddAssign};

use aoc_common::day::Day;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const MAS: [char; 3] = ['M', 'A', 'S'];
const SAM: [char; 3] = ['S', 'A', 'M'];

const DIRECTIONS: [Direction; 8] = [
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
    Direction::RightDown,
    Direction::RightUp,
    Direction::LeftDown,
    Direction::LeftUp,
];

#[derive(Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Position {
    fn in_direction(self, dir: Direction) -> impl Iterator<Item = Position> {
        std::iter::successors(Some(self), move |&pos| Some(pos + dir.offset()))
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl Direction {
    fn offset(self) -> Position {
        match self {
            Direction::Right => Position { x: 1, y: 0 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::RightDown => Position { x: 1, y: 1 },
            Direction::RightUp => Position { x: 1, y: -1 },
            Direction::LeftDown => Position { x: -1, y: 1 },
            Direction::LeftUp => Position { x: -1, y: -1 },
        }
    }
}

struct WordSearch {
    grid: Vec<Vec<char>>,
    y_rows: usize,
    x_cols: usize,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        WordSearch {
            y_rows: grid.len(),
            x_cols: grid[0].len(),
            grid,
        }
    }

    fn at(&self, pos: Position) -> Option<char> {
        let x: usize = pos.x.try_into().ok()?;
        let y: usize = pos.y.try_into().ok()?;

        let row = self.grid.get(y)?;
        row.get(x).copied()
    }

    fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.y_rows).flat_map(|y| {
            (0..self.x_cols).map(move |x| Position {
                x: x as i32,
                y: y as i32,
            })
        })
    }

    fn check_in_direction(&self, pos: Position, dir: Direction, word: &[char]) -> bool {
        pos.in_direction(dir)
            .zip(word.iter())
            .all(|(pos, &expected)| self.at(pos) == Some(expected))
    }

    fn check_in_all_directions(&self, pos: Position) -> u8 {
        DIRECTIONS
            .into_iter()
            .filter(|&dir| self.check_in_direction(pos, dir, &XMAS))
            .count() as u8
    }

    fn check_sam_or_mas(&self, pos: Position, dir: Direction) -> bool {
        self.check_in_direction(pos, dir, &MAS) || self.check_in_direction(pos, dir, &SAM)
    }
}

#[derive(Default)]
pub struct Day04 {}

impl Day for Day04 {
    type P1 = u32;
    type P2 = u32;

    fn solve_part_1(&self, input: &str) -> Self::P1 {
        let word_search = WordSearch::new(input);

        word_search
            .positions()
            .map(|pos| word_search.check_in_all_directions(pos) as u32)
            .sum()
    }

    fn solve_part_2(&self, input: &str) -> Self::P2 {
        let word_search = WordSearch::new(input);

        word_search
            .positions()
            .filter(|&pos| {
                let first_line = word_search.check_sam_or_mas(pos, Direction::RightDown);
                let second_line = word_search
                    .check_sam_or_mas(pos + Position { x: 2, y: 0 }, Direction::LeftDown);

                first_line && second_line
            })
            .count() as u32
    }

    fn get_day(&self) -> u8 {
        4
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

        let day = Day04::default();
        let part_1 = day.solve_part_1(&input);

        assert_eq!(part_1, 18)
    }

    #[test]
    fn test_example_part_2() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

        let day = Day04::default();
        let part_2 = day.solve_part_2(&input);

        assert_eq!(part_2, 9)
    }
}
