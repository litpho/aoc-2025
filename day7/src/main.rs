use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult, Parser,
};

const DATA: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let mut grid = result?;

    let (took, result) = took::took(|| part_one(&mut grid));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let grid = result?;

    let (took, result) = took::took(|| part_two(&grid));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(grid: &mut Grid) -> usize {
    let mut count = 0;
    let mut next = vec![grid.start];
    loop {
        next = next
            .iter()
            .flat_map(|pos| {
                let (n, c) = beam_down(grid, *pos);
                count += c;
                n
            })
            .collect();
        if next.is_empty() {
            break;
        }
    }

    // grid.visualize();

    count
}

fn beam_down(grid: &mut Grid, pos: (usize, usize)) -> (Vec<(usize, usize)>, usize) {
    if pos.1 == grid.size.1 - 1 {
        return (vec![], 0);
    }
    match grid.values[pos.1 + 1][pos.0] {
        Value::Beam => {
            // already set by a different branch
            (vec![], 0)
        }
        Value::Empty => {
            grid.values[pos.1 + 1][pos.0] = Value::Beam;
            (vec![(pos.0, pos.1 + 1)], 0)
        }
        Value::Splitter => {
            grid.values[pos.1 + 1][pos.0 - 1] = Value::Beam;
            grid.values[pos.1 + 1][pos.0 + 1] = Value::Beam;
            (vec![(pos.0 - 1, pos.1 + 1), (pos.0 + 1, pos.1 + 1)], 1)
        }
        Value::Start => unreachable!(),
    }
}

fn part_two(_grid: &Grid) -> usize {
    todo!()
}

#[derive(Debug)]
struct Grid {
    values: Vec<Vec<Value>>,
    size: (usize, usize),
    start: (usize, usize),
}

impl Grid {
    fn visualize(&self) {
        for row in &self.values {
            for value in row {
                match value {
                    Value::Beam => print!("|"),
                    Value::Empty => print!("."),
                    Value::Start => print!("S"),
                    Value::Splitter => print!("^"),
                }
            }
            println!();
        }
    }
}

impl From<Vec<Vec<Value>>> for Grid {
    fn from(values: Vec<Vec<Value>>) -> Grid {
        let size = (values[0].len(), values.len());
        let start = values
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .position(|val| Value::Start.eq(val))
                    .map(|x| (x, y))
            })
            .unwrap();

        Grid {
            values,
            size,
            start,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Beam,
    Empty,
    Start,
    Splitter,
}

fn parse(input: &'static str) -> Result<Grid> {
    let (_, grid) = parse_input(input)?;

    Ok(grid)
}

fn parse_input(input: &str) -> IResult<&str, Grid> {
    map(parse_lines, |values| values.into()).parse(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Value>>> {
    separated_list1(line_ending, parse_line).parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Value>> {
    many1(alt((parse_empty, parse_start, parse_splitter))).parse(input)
}

fn parse_empty(input: &str) -> IResult<&str, Value> {
    value(Value::Empty, tag(".")).parse(input)
}

fn parse_start(input: &str) -> IResult<&str, Value> {
    value(Value::Start, tag("S")).parse(input)
}

fn parse_splitter(input: &str) -> IResult<&str, Value> {
    value(Value::Splitter, tag("^")).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_parse_one_testdata() -> Result<()> {
        let mut grid = parse(TESTDATA)?;

        let res = part_one(&mut grid);
        assert_eq!(res, 21);

        Ok(())
    }

    // #[test]
    // fn test_parse_one() -> Result<()> {
    //     let problems = parse(DATA)?;
    //
    //     let res = part_one(&problems);
    //     assert_eq!(res, 4449991244405);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_parse_two_testdata() -> Result<()> {
    //     let problems = parse_two(TESTDATA)?;
    //
    //     let res = part_two(&problems);
    //     assert_eq!(res, 3263827);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn test_parse_two() -> Result<()> {
    //     let problems = parse_two(DATA)?;
    //
    //     let res = part_two(&problems);
    //     assert_eq!(res, 9348430857627);
    //
    //     Ok(())
    // }
}
