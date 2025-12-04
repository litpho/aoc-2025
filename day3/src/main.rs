use anyhow::Result;
use nom::{
    character::complete::{line_ending, satisfy},
    combinator::map_res,
    multi::{many1, separated_list1},
    AsChar, IResult, Parser,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result?;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &[Vec<u16>]) -> u64 {
    input.iter().map(|row| calculate_line(row, 2)).sum::<u64>()
}

fn part_two(input: &[Vec<u16>]) -> u64 {
    input.iter().map(|row| calculate_line(row, 12)).sum::<u64>()
}

fn calculate_line(input: &[u16], length: usize) -> u64 {
    let mut vec = input.to_vec();
    'outer: while vec.len() > length {
        for i in 1..vec.len() {
            if vec[i - 1] < vec[i] {
                vec.remove(i - 1);
                continue 'outer;
            }
        }

        vec.remove(get_smallest_loc(&vec));
    }

    vec.iter()
        .enumerate()
        .map(|(pos, i)| *i as u64 * 10u64.pow((length - 1 - pos) as u32))
        .sum::<u64>()
}

fn get_smallest_loc(input: &[u16]) -> usize {
    input
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
        .0
}

fn parse(input: &'static str) -> Result<Vec<Vec<u16>>> {
    let (_, result) = parse_input(input)?;

    Ok(result)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u16>>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<u16>> {
    many1(map_res(satisfy(|c| c.is_dec_digit()), |c| {
        u16::try_from(c).map(|x| x - 48)
    }))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_parse_one_testdata() -> Result<()> {
        let res = part_one(&parse(TESTDATA)?);
        assert_eq!(res, 357);

        Ok(())
    }

    #[test]
    fn test_parse_one() -> Result<()> {
        let res = part_one(&parse(DATA)?);
        assert_eq!(res, 17408);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let input = parse(TESTDATA)?;
        assert_eq!(3121910778619, part_two(&input));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse(DATA)?;
        assert_eq!(172740584266849, part_two(&input));

        Ok(())
    }
}
