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
    input.iter().map(|row| calculate_line(row)).sum::<u64>()
}

fn calculate_line(input: &[u16]) -> u64 {
    let length = input.len();
    let mut digits: [usize; 2] = [0, 1];
    for (i, val) in input.iter().enumerate().skip(2) {
        if input[digits[1]] > input[digits[0]] && i < length {
            digits[0] = digits[1];
            digits[1] = i;
        }
        if val > &input[digits[1]] {
            digits[1] = i;
        }
    }

    (input[digits[0]] * 10 + input[digits[1]]) as u64
}

fn part_two(input: &[Vec<u16>]) -> u64 {
    input
        .iter()
        .skip(2)
        // .take(1)
        .map(|row| calculate_line_two(row))
        .inspect(|val| println!("Val: {val}"))
        .sum::<u64>()
}

fn calculate_line_two(input: &[u16]) -> u64 {
    let length = input.len();
    let mut digits: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut moves = 0;
    for (i, val) in input.iter().enumerate().skip(1) {
        let mut found = false;
        println!("i: {i}");
        for j in 0..11 {
            println!("j: {j}");
            if input[digits[j + 1]] >= input[digits[j]] {
                println!(
                    "Moving {}({}) over {}({})",
                    input[digits[j + 1]],
                    digits[j + 1],
                    input[digits[j]],
                    digits[j]
                );
                digits[j] = digits[j + 1];
                found = true;
            } else if found {
                println!("{} {}", digits[j], digits[j + 1]);
                digits[j] = digits[j + 1];
            }
        }
        debug(input, &mut digits);
        println!("length: {length}, {moves} >= {}", length - 12);
        if (val > &input[digits[11]] || found) && digits[11] < length - 1 {
            println!("digits[11] {}", digits[11]);
            digits[11] += 1;
            moves += 1;
            if moves >= length - 12 {
                println!("break {digits:?}");
                break;
            }
        }
        println!("{digits:?}");
    }

    println!("{digits:?}");

    debug(input, &mut digits);

    digits
        .iter()
        .enumerate()
        .map(|(pos, i)| input[*i] as u64 * 10u64.pow((11 - pos) as u32))
        .sum::<u64>()
}

fn debug(input: &[u16], digits: &mut [usize; 12]) {
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}",
        input[digits[0]],
        input[digits[1]],
        input[digits[2]],
        input[digits[3]],
        input[digits[4]],
        input[digits[5]],
        input[digits[6]],
        input[digits[7]],
        input[digits[8]],
        input[digits[9]],
        input[digits[10]],
        input[digits[11]]
    );
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
        let vec = parse(TESTDATA)?;

        println!("{:?}", vec);

        let res = part_one(&vec);
        assert_eq!(res, 357);

        Ok(())
    }

    #[test]
    fn test_parse_one() -> Result<()> {
        let res = part_one(&parse(DATA)?);
        assert_eq!(res, 17408);

        Ok(())
    }

    // #[test]
    // fn test_part_two_testdata() -> Result<()> {
    //     let input = parse(TESTDATA)?;
    //     assert_eq!(3121910778619, part_two(&input));
    //
    //     Ok(())
    // }

    // #[test]
    // fn test_part_two() -> Result<()> {
    //     let input = parse(DATA)?;
    //     assert_eq!(69553832684, part_two(&input));
    //
    //     Ok(())
    // }
}
