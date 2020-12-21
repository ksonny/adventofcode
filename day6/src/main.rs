use nom::{
	character::complete::{newline, one_of},
	combinator::map,
	multi::{count, many1, separated_list1},
	IResult,
};
use std::{
	collections::BTreeMap,
	io::{self, Read},
	str,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Answer {
	Answer(char),
}

enum Group {
	Group(usize, BTreeMap<Answer, usize>),
}

fn parse_answer(i: &[u8]) -> IResult<&[u8], Answer> {
	map(one_of("abcdefghijklmnopqrstuvwxyz"), |c| Answer::Answer(c))(i)
}

fn parse_group(i: &[u8]) -> IResult<&[u8], Group> {
	map(separated_list1(newline, many1(parse_answer)), |v| {
		let members = v.len();
		let mut map = BTreeMap::new();
		for a in v.into_iter().flatten() {
			map.entry(a).and_modify(|v| *v += 1).or_insert(1);
		}
		Group::Group(members, map)
	})(i)
}

fn parse_groups(i: &[u8]) -> IResult<&[u8], Vec<Group>> {
	separated_list1(count(newline, 2), parse_group)(i)
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = Vec::new();
	stdin
		.lock()
		.read_to_end(&mut buffer)
		.expect("Failed to read from stdin");

	let (rest, groups) = parse_groups(&buffer).expect("Failed to parse input");
	let anyone_answered_count = groups
		.iter()
		.fold(0, |acc, Group::Group(_, a)| acc + a.len());
	let everyone_answered_count =
		groups.iter().fold(0, |acc_a, Group::Group(members, a)| {
			acc_a
				+ a.iter().fold(0, |acc_b, (_, &c)| {
					if c == *members {
						acc_b + 1
					} else {
						acc_b
					}
				})
		});

	match rest {
		b"" | b"\n" => {
			println!(
				"Answer part1: {} among {} groups",
				anyone_answered_count,
				groups.len()
			);
			println!("Answer part2: {}", everyone_answered_count,);
		}
		_ => println!(
			"Failed to consume input: {}",
			str::from_utf8(rest).unwrap()
		),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_answer_a() {
		let (rest, r) = parse_answer(b"a").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Answer::Answer('a'), r);
	}

	#[test]
	fn test_parse_group_a() {
		let (rest, Group::Group(members, r)) =
			parse_group(b"a").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(1, members);
		assert_eq!(
			vec![(Answer::Answer('a'), 1)],
			r.into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_parse_group_aabc() {
		let (rest, Group::Group(members, r)) =
			parse_group(b"aab\nc").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(2, members);
		assert_eq!(
			vec![
				(Answer::Answer('a'), 2),
				(Answer::Answer('b'), 1),
				(Answer::Answer('c'), 1),
			],
			r.into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_parse_group_abc() {
		let (rest, Group::Group(members, r)) =
			parse_group(b"ab\nc").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(2, members);
		assert_eq!(
			vec![
				(Answer::Answer('a'), 1),
				(Answer::Answer('b'), 1),
				(Answer::Answer('c'), 1),
			],
			r.into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_parse_group_fekdcbayqxnwvh() {
		let (rest, Group::Group(members, r)) =
			parse_group(b"fekdcbayqxnwvh").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(1, members);
		assert_eq!(
			vec![
				(Answer::Answer('a'), 1),
				(Answer::Answer('b'), 1),
				(Answer::Answer('c'), 1),
				(Answer::Answer('d'), 1),
				(Answer::Answer('e'), 1),
				(Answer::Answer('f'), 1),
				(Answer::Answer('h'), 1),
				(Answer::Answer('k'), 1),
				(Answer::Answer('n'), 1),
				(Answer::Answer('q'), 1),
				(Answer::Answer('v'), 1),
				(Answer::Answer('w'), 1),
				(Answer::Answer('x'), 1),
				(Answer::Answer('y'), 1),
			],
			r.into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_parse_group_abc_not_d() {
		let (rest, Group::Group(members, r)) =
			parse_group(b"ab\nc\n\nd").expect("Failed to parse input");
		assert_eq!(b"\n\nd", rest, "Unexpected remaining input");
		assert_eq!(2, members);
		assert_eq!(
			vec![
				(Answer::Answer('a'), 1),
				(Answer::Answer('b'), 1),
				(Answer::Answer('c'), 1),
			],
			r.into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn test_parse_groups_ab_ac() {
		let (rest, r) =
			parse_groups(b"ab\n\nac").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(
			vec![
				vec![(Answer::Answer('a'), 1), (Answer::Answer('b'), 1)],
				vec![(Answer::Answer('a'), 1), (Answer::Answer('c'), 1)],
			],
			r.into_iter()
				.map(|Group::Group(_, v)| v.into_iter().collect::<Vec<_>>())
				.collect::<Vec<_>>()
		);
	}
}
