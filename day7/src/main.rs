use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha1, digit1, newline, space0},
	combinator::{map, map_res, recognize},
	multi::{many1, separated_list1},
	sequence::{pair, separated_pair, terminated},
	IResult,
};
use std::{
	collections::{BTreeMap, BTreeSet},
	io::{self, Read},
	str,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Bag<'a> {
	Bag(&'a str),
}

#[derive(Debug)]
struct BagRuleLookup<'a> {
	map: BTreeMap<Bag<'a>, Vec<(usize, Bag<'a>)>>,
}

fn parse_count(i: &[u8]) -> IResult<&[u8], usize> {
	map_res(map_res(recognize(digit1), str::from_utf8), str::parse)(i)
}

fn parse_bag(i: &[u8]) -> IResult<&[u8], Bag> {
	map(
		terminated(
			map_res(
				recognize(separated_pair(alpha1, tag(" "), alpha1)),
				str::from_utf8,
			),
			alt((tag(" bags"), tag(" bag"))),
		),
		|n| Bag::Bag(n),
	)(i)
}

fn parse_rule(i: &[u8]) -> IResult<&[u8], (Bag, Vec<(usize, Bag)>)> {
	pair(
		terminated(parse_bag, tag(" contain ")),
		terminated(
			alt((
				map(tag("no other bags"), |_| vec![]),
				separated_list1(
					terminated(tag(","), space0),
					separated_pair(parse_count, tag(" "), parse_bag),
				),
			)),
			tag("."),
		),
	)(i)
}

fn parse_rule_lookup(
	i: &[u8],
) -> IResult<&[u8], BTreeMap<Bag, Vec<(usize, Bag)>>> {
	map(many1(terminated(parse_rule, newline)), |m| {
		m.into_iter().collect()
	})(i)
}

fn into_revers_rule_lookup<'a>(
	map: &BTreeMap<Bag<'a>, Vec<(usize, Bag<'a>)>>,
) -> BTreeMap<Bag<'a>, Vec<(usize, Bag<'a>)>> {
	map.iter().fold(BTreeMap::new(), |mut acc, (&b, bgs)| {
		for &(n, c) in bgs {
			acc.entry(c)
				.and_modify(|e| {
					e.push((n, b));
				})
				.or_insert(vec![(n, b)]);
		}
		acc
	})
}

fn create_set<'a>(
	map: &'a BTreeMap<Bag, Vec<(usize, Bag)>>,
	set: BTreeSet<Bag<'a>>,
	bag: Bag,
) -> BTreeSet<Bag<'a>> {
	if let Some(bgs) = map.get(&bag) {
		bgs.iter().fold(set, |mut acc, &(_, b)| {
			if acc.insert(b) {
				create_set(map, acc, b)
			} else {
				acc
			}
		})
	} else {
		set
	}
}

fn get_content<'a>(
	map: &'a BTreeMap<Bag, Vec<(usize, Bag)>>,
	bag: Bag,
) -> Vec<(usize, Bag<'a>)> {
	if let Some(bgs) = map.get(&bag) {
		let mut acc = bgs.to_vec();
		for &(n, b) in bgs {
			acc.append(
				&mut get_content(map, b)
					.into_iter()
					.map(|(m, c)| (n * m, c))
					.collect(),
			);
		}
		acc
	} else {
		vec![]
	}
}

impl BagRuleLookup<'_> {
	fn count_bags_containing(&self, b: Bag) -> usize {
		let reverse_map = into_revers_rule_lookup(&self.map);
		create_set(&reverse_map, BTreeSet::new(), b).len()
	}

	fn count_bag_content(&self, b: Bag) -> usize {
		get_content(&self.map, b)
			.iter()
			.fold(0, |acc, (c, _)| acc + c)
	}
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = Vec::new();
	stdin
		.lock()
		.read_to_end(&mut buffer)
		.expect("Failed to read from stdin");

	let rules = parse_rule_lookup(&buffer)
		.map(|(_, map)| BagRuleLookup { map })
		.expect("Failed to parse rule set");
	let part1_count = rules.count_bags_containing(Bag::Bag("shiny gold"));
	let part2_count = rules.count_bag_content(Bag::Bag("shiny gold"));

	println!("Answer part1: {}", part1_count);
	println!("Answer part2: {}", part2_count);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_count() {
		let (rest, r) = parse_count(b"10").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(10, r);
	}

	#[test]
	fn test_parse_bag() {
		let (rest, r) =
			parse_bag(b"shiny gold bag").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Bag::Bag("shiny gold"), r);
	}

	#[test]
	fn test_parse_bag_plural() {
		let (rest, r) =
			parse_bag(b"shiny gold bags").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Bag::Bag("shiny gold"), r);
	}

	#[test]
	fn test_parse_rule_contains_none() {
		let (rest, r) = parse_rule(b"faded blue bags contain no other bags.")
			.expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!((Bag::Bag("faded blue"), vec![]), r);
	}

	#[test]
	fn test_parse_rule_contains_one() {
		let (rest, r) =
			parse_rule(b"vibrant plum bags contain 5 faded blue bags.")
				.expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(
			(Bag::Bag("vibrant plum"), vec![(5, Bag::Bag("faded blue"))]),
			r
		);
	}

	#[test]
	fn test_parse_rule_contains_two() {
		let (rest, r) =
			parse_rule(b"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(
			(
				Bag::Bag("vibrant plum"),
				vec![
					(5, Bag::Bag("faded blue")),
					(6, Bag::Bag("dotted black"))
				]
			),
			r
		);
	}
}
