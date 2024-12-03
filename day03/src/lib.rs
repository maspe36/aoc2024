use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, digit1};
use nom::combinator::map;
use nom::multi::{fold_many0, many_till};
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub fn number(s: &str) -> IResult<&str, usize> {
    map(digit1, |d: &str| d.parse::<usize>().unwrap())(s)
}

pub fn pair(s: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(number, char(','), number)(s)
}

pub fn mul(s: &str) -> IResult<&str, (usize, usize)> {
    delimited(tag("mul("), pair, tag(")"))(s)
}

pub fn do_or_dont_anychar<'a>(s: &'a str, enabled: Rc<RefCell<bool>>) -> IResult<&'a str, ()> {
    // Define a parser for "do()" that sets `enabled` to true
    let parse_do = map(tag("do()"), |_| {
        *enabled.borrow_mut() = true;
    });

    // Define a parser for "don't()" that sets `enabled` to false
    let parse_dont = map(tag("don't()"), |_| {
        *enabled.borrow_mut() = false;
    });

    // Define a parser for any single character
    let parse_anychar = map(anychar, |_| {});

    // Combine the parsers to first attempt "do()" or "don't()" and fallback to anychar
    alt((parse_do, parse_dont, parse_anychar))(s)
}

pub fn parse01(s: &str) -> Vec<(usize, usize)> {
    fold_many0(
        many_till(anychar, mul),
        Vec::new,
        |mut acc: Vec<_>, (_, item)| {
            acc.push(item);
            acc
        },
    )(s)
    .unwrap()
    .1
}

pub fn parse02(s: &str) -> Vec<(usize, usize)> {
    let enabled = Rc::new(RefCell::new(true));

    // The compiler suggested this for me. I had a subtle drop error and honestly, I have no idea why.
    let x = fold_many0(
        // Okay, now `mul` needs to be a different parser that handles the `do()` and `don't()`
        many_till(|s| do_or_dont_anychar(s, enabled.clone()), mul),
        Vec::new,
        |mut acc: Vec<_>, (_, item)| {
            // Only "count" the `mul()` if its enabled
            if *enabled.borrow() {
                acc.push(item);
            }

            acc
        },
    )(s)
    .unwrap()
    .1;
    x
}

pub fn part1() -> usize {
    let input = fs::read_to_string(helpers::workspace_dir().join("day03/input.txt")).unwrap();
    let muls = parse01(&input);

    muls.into_iter().map(|(l, r)| l * r).sum()
}

pub fn part2() -> usize {
    let input = fs::read_to_string(helpers::workspace_dir().join("day03/input.txt")).unwrap();
    let muls = parse02(&input);

    muls.into_iter().map(|(l, r)| l * r).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Part 1
    // 2*4 + 5*5 + 11*8 + 8*5 == 161

    // Part 2
    // 2*4 + 8*5 == 48
    // Only the most recent do() or don't() instruction applies.
    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse01() {
        let result = parse01(EXAMPLE);

        println!("{:?}", result);
    }

    #[test]
    fn test_parse02() {
        let result = parse02(EXAMPLE_2);

        println!("{:?}", result);
    }

    #[test]
    fn test_part1() {
        let result = part1();
        println!("{}", result);
    }

    #[test]
    fn test_part2() {
        let result = part2();
        println!("{}", result);
    }
}
