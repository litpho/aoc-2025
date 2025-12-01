use std::str::FromStr;

const DATA: &str = include_str!("input.txt");

fn main() {
    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_one(&input));
    println!("Result part one: {result}");
    println!("Time spent: {took}");

    let (took, result) = took::took(|| parse(DATA));
    println!("Time spent parsing: {took}");
    let input = result;

    let (took, result) = took::took(|| part_two(&input));
    println!("Result part two: {result}");
    println!("Time spent: {took}");
}

fn part_one(input: &[isize]) -> usize {
    input
        .iter()
        .fold((50isize, 0usize), |(direction, count), val| {
            let new_direction = direction + val;
            if direction % 100 == 0 {
                (new_direction, count + 1)
            } else {
                (new_direction, count)
            }
        })
        .1
}

fn part_two(input: &[isize]) -> usize {
    let mut count = 0;
    let mut direction = 50isize;
    for val in input {
        let (sanitized_val, cnt) = sanitize(direction, *val);
        count += cnt;
        if sanitized_val == 0 {
            println!("=====================================");
        }
        println!("{direction} {val} {sanitized_val} {cnt} {count}");

        direction = sanitized_val;
    }

    count
}

fn sanitize(direction: isize, val: isize) -> (isize, usize) {
    let mut x = direction + val;
    let mut count = 0;
    if x % 100 == 0 && val < 0 {
        count += 1;
    }
    while x < 0 {
        x += 100;
        count += 1;
    }
    while x >= 100 {
        x -= 100;
        count += 1;
    }
    if direction == 0 && val < 0 {
        count -= 1;
    }
    if count == 0 && x == 0 {
        count = 1;
    }

    (x, count)
}

fn parse(input: &str) -> Vec<isize> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> isize {
    let (letter, value) = line.split_at(1);
    if letter.eq("L") {
        -isize::from_str(value).unwrap()
    } else {
        isize::from_str(value).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = include_str!("test.txt");

    #[test]
    fn test_part_one_testdata() {
        assert_eq!(3, part_one(&parse(TESTDATA)));
    }

    #[test]
    fn test_part_one() {
        let input = parse(DATA);
        assert_eq!(1123, part_one(&input));
    }

    #[test]
    fn test_part_two_testdata() {
        let input = parse(TESTDATA);
        assert_eq!(6, part_two(&input));
    }

    #[test]
    fn test_part_two() {
        let input = parse(DATA);
        assert_eq!(6695, part_two(&input));
    }
}
