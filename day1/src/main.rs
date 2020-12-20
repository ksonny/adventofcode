use io::Lines;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::IResult;
use std::{
    env,
    io::{self, BufRead},
};

fn number(i: &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(i)
}

fn read_input<R: BufRead>(lines: Lines<R>) -> Vec<u64> {
    lines
        .filter_map(|r| r.map_or(None, |s| Some(s)))
        .filter_map(|s| number(&s).map_or(None, |(_, n)| Some(n)))
        .collect::<Vec<_>>()
}

fn find_day1_part1_num(input: &mut Vec<u64>, sum: u64) -> Option<u64> {
    input.iter().enumerate().find_map(|(i, m)| {
        input
            .iter()
            .skip(i)
            .find_map(|&n| if (m + n) == sum { Some(m * n) } else { None })
    })
}

fn find_day1_part2_num(input: &mut Vec<u64>, sum: u64) -> Option<u64> {
    input.iter().enumerate().find_map(|(i, m)| {
        input.iter().enumerate().skip(i).find_map(|(j, n)| {
            input.iter().skip(j).find_map(|&o| {
                if (m + n + o) == sum {
                    Some(m * n * o)
                } else {
                    None
                }
            })
        })
    })
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let sum = args
        .get(1)
        .and_then(|s| number(s).map(|(_, u)| Some(u)).unwrap_or(None))
        .unwrap_or(2020);

    let input = read_input(io::stdin().lock().lines());
    let part1_answer = find_day1_part1_num(&mut input.clone(), sum);
    let part2_answer = find_day1_part2_num(&mut input.clone(), sum);

    if let Some(x) = part1_answer {
        println!("part1 answer: {}", x);
    }
    if let Some(x) = part2_answer {
        println!("part2 answer: {}", x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_read_input() {
        let mut buffer = Vec::new();
        buffer.write_all(b"0\n1\n2\n3\n4\n").unwrap();

        let input = read_input(buffer.lines());
        assert_eq!(input, vec![0, 1, 2, 3, 4]);
    }
}
