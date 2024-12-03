use aoc_common::day::Day;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::tuple, IResult,
};
use once_cell::sync::Lazy;
use regex::Regex;

static PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").expect("should be a valid regex"));
static EXTENDED_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").expect("should be a valid regex")
});

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

struct InstructionProcessor {
    do_state: bool,
    result: u32,
}

impl InstructionProcessor {
    fn new() -> Self {
        InstructionProcessor {
            do_state: true,
            result: 0,
        }
    }

    fn process(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Do => self.do_state = true,
            Instruction::Dont => self.do_state = false,
            Instruction::Mul(a, b) if self.do_state => self.result += a * b,
            Instruction::Mul(_, _) => {}
        }
    }
}

#[derive(Default)]
pub struct Day03 {}

impl Day for Day03 {
    type P1 = u32;
    type P2 = u32;

    fn solve_part_1(&self, input: &str) -> Self::P1 {
        let mut processor = InstructionProcessor::new();
        extract_mul_instructions(input).for_each(|instruction| processor.process(&instruction));

        processor.result
    }

    fn solve_part_2(&self, input: &str) -> Self::P2 {
        let mut processor = InstructionProcessor::new();
        extract_valid_instructions(input).for_each(|instruction| processor.process(&instruction));

        processor.result
    }

    fn get_day(&self) -> u8 {
        3
    }
}

fn extract_mul_instructions(instructions: &str) -> impl Iterator<Item = Instruction> + '_ {
    PATTERN.captures_iter(instructions).map(|capture| {
        let (_, [fst, snd]) = capture.extract();
        Instruction::Mul(
            fst.parse().expect("should be a valid integer"),
            snd.parse().expect("should be a valid integer"),
        )
    })
}
fn extract_valid_instructions(instructions: &str) -> impl Iterator<Item = Instruction> + '_ {
    EXTENDED_PATTERN.captures_iter(instructions).map(|capture| {
        let (_, [instruction]) = capture.extract();
        let (_, instruction) =
            parse_instruction(instruction).expect("should be a valid instruction");

        instruction
    })
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |str: &str| str.parse())(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b, _)) =
        tuple((tag("mul("), parse_int, tag(","), parse_int, tag(")")))(input)?;

    Ok((input, Instruction::Mul(a, b)))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_mul, parse_do, parse_dont))(input)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

        let day = Day03::default();
        let part_1 = day.solve_part_1(&input);

        assert_eq!(part_1, 161)
    }

    #[test]
    fn test_example_part_2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

        let day = Day03::default();
        let part_2 = day.solve_part_2(&input);

        assert_eq!(part_2, 48)
    }
}
