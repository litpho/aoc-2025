use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::{complete, complete::line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult, Parser,
};
use std::ops::RangeInclusive;

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let (ranges, ingredients) = result?;

    let (took, result) = took::took(|| part_one(&ranges, &ingredients));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let (ranges, _) = result?;

    let (took, result) = took::took(|| part_two(&ranges));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(ranges: &[RangeInclusive<u64>], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn part_two(ranges: &[RangeInclusive<u64>]) -> u64 {
    ranges.iter().map(|r| 1 + (r.end() - r.start())).sum()
}

fn parse(input: &'static str) -> Result<(Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (_, (ranges, ingredients)) = parse_list(input)?;

    let ranges = merge_ranges(ranges);

    Ok((ranges, ingredients))
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    ranges.sort_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())));
    for i in (0..ranges.len() - 1).rev() {
        if *ranges[i].end() >= ranges[i + 1].start() - 1u64 {
            if ranges[i + 1].end() > ranges[i].end() {
                // if not fully enclosed
                ranges[i] = *ranges[i].start()..=*ranges[i + 1].end();
            }
            ranges.remove(i + 1);
        }
    }

    ranges
}

fn parse_list(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    separated_pair(
        parse_ranges,
        pair(line_ending, line_ending),
        parse_ingredients,
    )
    .parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(line_ending, parse_range).parse(input)
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    map(
        separated_pair(complete::u64, tag("-"), complete::u64),
        |(start, end)| start..=end,
    )
    .parse(input)
}

fn parse_ingredients(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(line_ending, complete::u64).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_parse_one_testdata() -> Result<()> {
        let (ranges, ingredients) = parse(TESTDATA)?;

        let res = part_one(&ranges, &ingredients);
        assert_eq!(res, 3);

        Ok(())
    }

    #[test]
    fn test_parse_one() -> Result<()> {
        let (ranges, ingredients) = parse(DATA)?;

        let res = part_one(&ranges, &ingredients);
        assert_eq!(res, 517);

        Ok(())
    }

    #[test]
    fn test_parse_two_testdata() -> Result<()> {
        let (ranges, _) = parse(TESTDATA)?;

        let res = part_two(&ranges);
        assert_eq!(res, 14);

        Ok(())
    }

    #[test]
    fn test_parse_two() -> Result<()> {
        let (ranges, _) = parse(DATA)?;

        let res = part_two(&ranges);
        assert_eq!(res, 336173027056994);

        Ok(())
    }
}
