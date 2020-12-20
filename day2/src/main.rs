use io::Lines;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, digit1, multispace0},
    sequence::{preceded, terminated, tuple},
};
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Entry {
    upper: u64,
    lower: u64,
    char: char,
    password: String,
}

fn number1(i: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(i)
}

fn password1(i: &str) -> IResult<&str, &str> {
    alphanumeric1(i)
}

fn entry(i: &str) -> IResult<&str, Entry> {
    map(
        tuple((
            terminated(number1, tag("-")),
            terminated(number1, multispace0),
            terminated(anychar, tag(":")),
            preceded(multispace0, password1),
        )),
        |(l, u, c, p)| Entry {
            lower: l,
            upper: u,
            char: c,
            password: p.into(),
        },
    )(i)
}

fn read_input<R: BufRead>(lines: Lines<R>) -> impl Iterator<Item = Entry> {
    lines
        .filter_map(|r| r.map_or(None, |s| Some(s)))
        .filter_map(|s| entry(&s).map_or(None, |(_, n)| Some(n)))
}

fn valid_day2_part1(entry: &Entry) -> bool {
    let char_count = entry
        .password
        .chars()
        .fold(0, |acc, c| if c == entry.char { acc + 1 } else { acc });
    char_count >= entry.lower && char_count <= entry.upper
}

fn valid_day2_part2(entry: &Entry) -> bool {
    entry
        .password
        .char_indices()
        .filter(|&(i, _)| entry.lower as usize == i + 1 || entry.upper as usize == i + 1)
        .filter(|&(_, c)| c == entry.char)
        .count()
        == 1
}

fn main() {
    let stdin = io::stdin();
    let entries = read_input(stdin.lock().lines()).collect::<Vec<_>>();

    let valid_entries_count = entries.iter().filter(|e| valid_day2_part1(&e)).count();
    println!("part1 answer: {}", valid_entries_count);
    let valid_entries_count = entries.iter().filter(|e| valid_day2_part2(&e)).count();
    println!("part2 answer: {}", valid_entries_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_parse() {
        let input = "1-3 a: aaa";
        let e = Entry {
            lower: 1,
            upper: 3,
            char: 'a',
            password: "aaa".into(),
        };
        let (_, r) = entry(input).unwrap();
        assert_eq!(r, e);
    }

    #[test]
    fn test_valid_day2_trivial() {
        let entry = Entry {
            lower: 1,
            upper: 1,
            char: 'a',
            password: "a".into(),
        };
        assert!(valid_day2_part1(&entry));
    }

    #[test]
    fn test_valid_day2_between() {
        let entry = Entry {
            lower: 1,
            upper: 3,
            char: 'a',
            password: "aa".into(),
        };
        assert!(valid_day2_part1(&entry));
    }

    #[test]
    fn test_valid_day2_no_matching() {
        let entry = Entry {
            lower: 1,
            upper: 1,
            char: 'a',
            password: "b".into(),
        };
        assert!(!valid_day2_part1(&entry));
    }

    #[test]
    fn test_valid_day2_too_many() {
        let entry = Entry {
            lower: 1,
            upper: 1,
            char: 'a',
            password: "aa".into(),
        };
        assert!(!valid_day2_part1(&entry));
    }

    #[test]
    fn test_valid_day2_too_few() {
        let entry = Entry {
            lower: 3,
            upper: 3,
            char: 'a',
            password: "aa".into(),
        };
        assert!(!valid_day2_part1(&entry));
    }

    #[test]
    fn test_valid_day2_part2_a() {
        let entry = Entry {
            lower: 1,
            upper: 3,
            char: 'a',
            password: "abcde".into(),
        };
        assert!(valid_day2_part2(&entry));
    }

    #[test]
    fn test_valid_day2_part2_b() {
        let entry = Entry {
            lower: 1,
            upper: 3,
            char: 'b',
            password: "cdefg".into(),
        };
        assert!(!valid_day2_part2(&entry));
    }

    #[test]
    fn test_valid_day2_part2_c() {
        let entry = Entry {
            lower: 2,
            upper: 9,
            char: 'c',
            password: "ccccccccc".into(),
        };
        assert!(!valid_day2_part2(&entry));
    }
}
