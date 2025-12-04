use anyhow::Result;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult, Parser,
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
    let mut input = result?;

    let (took, result) = took::took(|| part_two(&mut input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");

    Ok(())
}

fn part_one(input: &Grid) -> usize {
    rolls_to_remove(input).len()
}

fn rolls_to_remove(input: &Grid) -> Vec<(usize, usize)> {
    input
        .data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, val)| {
                if *val && input.count_neighbours(x, y) < 4 {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>()
}

fn part_two(input: &mut Grid) -> usize {
    let mut count = 0;
    loop {
        let rolls = rolls_to_remove(input);
        if rolls.is_empty() {
            break;
        }
        count += rolls.len();

        input.remove(&rolls);
    }

    count
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<bool>>,
}

impl Grid {
    fn height(&self) -> usize {
        self.data.len()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        if y > 0 {
            if x > 0 && self.data[y - 1][x - 1] {
                count += 1;
            }
            if self.data[y - 1][x] {
                count += 1;
            }
            if x < self.width() - 1 && self.data[y - 1][x + 1] {
                count += 1;
            }
        }
        if x > 0 && self.data[y][x - 1] {
            count += 1;
        }
        if x < self.width() - 1 && self.data[y][x + 1] {
            count += 1;
        }
        if y < self.height() - 1 {
            if x > 0 && self.data[y + 1][x - 1] {
                count += 1;
            }
            if self.data[y + 1][x] {
                count += 1;
            }
            if x < self.width() - 1 && self.data[y + 1][x + 1] {
                count += 1;
            }
        }
        count
    }

    fn remove(&mut self, rolls: &[(usize, usize)]) {
        for (x, y) in rolls {
            self.data[*y][*x] = false;
        }
    }
}

fn parse(input: &'static str) -> Result<Grid> {
    let (_, result) = parse_grid(input)?;

    Ok(result)
}

fn parse_grid(input: &str) -> IResult<&str, Grid> {
    map(separated_list1(line_ending, parse_line), |data| Grid {
        data,
    })
    .parse(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<bool>> {
    many1(map(one_of(".@"), |c| c == '@')).parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_parse_one_testdata() -> Result<()> {
        let grid = parse(TESTDATA)?;

        grid.count_neighbours(0, 2);

        let res = part_one(&grid);
        assert_eq!(res, 13);

        Ok(())
    }

    #[test]
    fn test_parse_one() -> Result<()> {
        let grid = parse(DATA)?;

        let res = part_one(&grid);
        assert_eq!(res, 1464);

        Ok(())
    }

    #[test]
    fn test_parse_two_testdata() -> Result<()> {
        let mut grid = parse(TESTDATA)?;

        let res = part_two(&mut grid);
        assert_eq!(res, 43);

        Ok(())
    }

    #[test]
    fn test_parse_two() -> Result<()> {
        let mut grid = parse(DATA)?;

        let res = part_two(&mut grid);
        assert_eq!(res, 8409);

        Ok(())
    }
}
