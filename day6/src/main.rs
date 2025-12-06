use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self,
        line_ending,
        one_of,
        space0,
        space1
    },
    combinator::value,
    multi::{
        many1,
        separated_list1
    },
    sequence::{
        preceded,
        separated_pair
    },
    IResult,
    Parser
};
use std::str::FromStr;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse_one(DATA));
    println!("Time spent parsing: {took}");
    let problems = result?;

    let (took, result) = took::took(|| part_one(&problems));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse_two(DATA));
    println!("Time spent parsing: {took}");
    let problems = result?;

    let (took, result) = took::took(|| part_two(&problems));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(problems: &[Problem]) -> u64 {
    problems.iter().map(|p| p.calculate()).sum()
}

fn part_two(problems: &[Problem]) -> u64 {
    problems.iter().map(|p| p.calculate()).sum()
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Problem {
    fn calculate(&self) -> u64 {
        match self.operator {
            Operator::Add => self.numbers.iter().sum(),
            Operator::Mul => self.numbers.iter().product(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

fn parse_one(input: &'static str) -> Result<Vec<Problem>> {
    let (_, (lines, operators)) = parse_input(input, parse_lines_one)?;

    let problems = transpose_one(&lines, &operators);

    Ok(problems)
}

fn parse_two(input: &'static str) -> Result<Vec<Problem>> {
    let (_, (lines, mut operators)) = parse_input(input, parse_lines_two)?;

    operators.reverse();

    let problems = transpose_two(lines, &operators);

    Ok(problems)
}

fn transpose_one(lines: &[Vec<u64>], operators: &[Operator]) -> Vec<Problem> {
    let width = lines[0].len();
    let mut problems = vec![];
    for i in 0..width {
        let mut numbers = vec![];
        for line in lines {
            numbers.push(line[i]);
        }
        let operator = operators[i];
        problems.push(Problem { numbers, operator })
    }

    problems
}

fn transpose_two(lines: Vec<Vec<u64>>, operators: &[Operator]) -> Vec<Problem> {
    lines
        .into_iter()
        .enumerate()
        .map(|(i, numbers)| {
            let operator = operators[i];
            Problem { numbers, operator }
        })
        .collect()
}

fn parse_input(
    input: &str,
    line_parser: fn(&str) -> IResult<&str, Vec<Vec<u64>>>,
) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operator>)> {
    separated_pair(line_parser, line_ending, parse_operators).parse(input)
}

fn parse_lines_one(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    separated_list1(line_ending, parse_line_two).parse(input)
}

fn parse_lines_two(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (remainder, lines) =
        separated_list1(line_ending, many1(one_of("1234567890 "))).parse(input)?;
    let mut result: Vec<Vec<u64>> = vec![];
    let mut numbers: Vec<u64> = vec![];
    let longest = lines.iter().map(|line| line.len()).max().unwrap();
    let mut nums: Vec<String> = vec![];
    for i in (0..longest).rev() {
        for line in &lines {
            let digit = line.get(i);
            if let Some(digit) = digit
                && *digit != ' '
            {
                nums.push(digit.to_string());
            }
        }
        if nums.is_empty() {
            result.push(numbers.clone());
            numbers.clear();
        } else {
            numbers.push(u64::from_str(&nums.concat()).unwrap());
            nums.clear();
        }
    }
    if !numbers.is_empty() {
        result.push(numbers.clone());
    }

    Ok((remainder, result))
}

fn parse_line_two(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(space0, separated_list1(space1, complete::u64)).parse(input)
}

fn parse_operators(input: &str) -> IResult<&str, Vec<Operator>> {
    separated_list1(space1, alt((parse_add, parse_mul))).parse(input)
}

fn parse_add(input: &str) -> IResult<&str, Operator> {
    value(Operator::Add, tag("+")).parse(input)
}

fn parse_mul(input: &str) -> IResult<&str, Operator> {
    value(Operator::Mul, tag("*")).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_parse_one_testdata() -> Result<()> {
        let problems = parse_one(TESTDATA)?;

        let res = part_one(&problems);
        assert_eq!(res, 4277556);

        Ok(())
    }

    #[test]
    fn test_parse_one() -> Result<()> {
        let problems = parse_one(DATA)?;

        let res = part_one(&problems);
        assert_eq!(res, 4449991244405);

        Ok(())
    }

    #[test]
    fn test_parse_two_testdata() -> Result<()> {
        let problems = parse_two(TESTDATA)?;

        let res = part_two(&problems);
        assert_eq!(res, 3263827);

        Ok(())
    }

    #[test]
    fn test_parse_two() -> Result<()> {
        let problems = parse_two(DATA)?;

        let res = part_two(&problems);
        assert_eq!(res, 9348430857627);

        Ok(())
    }
}
