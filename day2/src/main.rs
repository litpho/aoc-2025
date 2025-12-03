use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::{char, complete},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::ops::RangeInclusive;

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

fn part_one(input: &[RangeInclusive<u64>]) -> u64 {
    find_with_filter(input, is_invalid_one)
}

fn part_two(input: &[RangeInclusive<u64>]) -> u64 {
    find_with_filter(input, is_invalid_two)
}

fn find_with_filter(input: &[RangeInclusive<u64>], filter: fn(&u64) -> bool) -> u64 {
    input
        .iter()
        .cloned()
        .flat_map(|r| r.filter(filter))
        .sum::<u64>()
}

fn is_invalid_one(id: &u64) -> bool {
    let id_string = id.to_string();
    let length = id_string.len();
    if !length.is_multiple_of(2) {
        return false;
    }

    let (left, right) = id_string.split_at(length / 2);

    left.eq(right)
}

fn is_invalid_two(id: &u64) -> bool {
    let id_string = id.to_string();
    let total = id_string.len();
    for chunk in 1..=total / 2 {
        if total.is_multiple_of(chunk) {
            let first = &id_string[0..chunk];
            if (1..(total / chunk))
                .into_iter()
                .all(|j| first.eq(&id_string[(j * chunk)..((j + 1) * chunk)]))
            {
                return true;
            }
        }
    }

    false
}

fn parse(input: &'static str) -> Result<Vec<RangeInclusive<u64>>> {
    let (_, result) = parse_list(input)?;

    Ok(result)
}

fn parse_list(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(tag(","), parse_range).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(
        separated_pair(complete::u64, char('-'), complete::u64),
        |(start, end)| start..=end,
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_parse_one_testdata() -> Result<()> {
        let res = part_one(&parse(TESTDATA)?);
        assert_eq!(res, 1227775554);

        Ok(())
    }

    #[test]
    fn test_parse_one() -> Result<()> {
        let res = part_one(&parse(DATA)?);
        assert_eq!(res, 53420042388);

        Ok(())
    }

    #[test]
    fn test_part_two_testdata() -> Result<()> {
        let input = parse(TESTDATA)?;
        assert_eq!(4174379265, part_two(&input));

        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<()> {
        let input = parse(DATA)?;
        assert_eq!(69553832684, part_two(&input));

        Ok(())
    }
}
