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
    input
        .iter()
        .fold((50isize, 0usize), |(direction, count), val| {
            let (new_direction, cnt) = sanitize(direction, *val);
            (new_direction, count + cnt)
        })
        .1
}

fn sanitize(direction: isize, val: isize) -> (isize, usize) {
    let mut target = direction + val;
    let mut count = 0;
    let rem = target.div_euclid(100);
    target += rem * -100;
    count += rem.unsigned_abs();
    if val < 0 {
        if direction == 0 {
            count -= 1;
        }
        if target == 0 {
            count += 1;
        }
    }
    if target == 0 && count == 0 {
        count = 1;
    }

    (target, count)
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
        assert_eq!(part_one(&parse(TESTDATA)), 3);
    }

    #[test]
    fn test_part_one() {
        let input = parse(DATA);
        assert_eq!(part_one(&input), 1123);
    }

    #[test]
    fn test_part_two_testdata() {
        let input = parse(TESTDATA);
        assert_eq!(part_two(&input), 6);
    }

    #[test]
    fn test_part_two() {
        let input = parse(DATA);
        assert_eq!(part_two(&input), 6695);
    }
}
