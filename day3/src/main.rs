use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{multispace0, newline},
	combinator::map,
	multi::{many1, separated_list1},
	sequence::preceded,
	IResult,
};
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum Terrain {
	Open,
	Tree,
}

#[derive(Debug)]
enum Map {
	Map(Vec<Vec<Terrain>>),
}

fn parse_terrain(i: &[u8]) -> IResult<&[u8], Terrain> {
	alt((
		map(tag("."), |_| Terrain::Open),
		map(tag("#"), |_| Terrain::Tree),
	))(i)
}

fn parse_map_line(i: &[u8]) -> IResult<&[u8], Vec<Terrain>> {
	preceded(multispace0, many1(parse_terrain))(i)
}

fn parse_map(i: &[u8]) -> IResult<&[u8], Map> {
	map(separated_list1(newline, parse_map_line), Map::Map)(i)
}

impl Map {
	fn lookup(&self, x: usize, y: usize) -> Option<&Terrain> {
		let Map::Map(map) = self;
		map.get(y).and_then(|line| line.get(x % line.len()))
	}

	fn height(&self) -> usize {
		let Map::Map(map) = self;
		map.len()
	}

	fn validate(&self) -> bool {
		let Map::Map(map) = self;
		let width = map
			.get(0)
			.map(|l| l.len())
			.expect("Failed to get length of first entry");
		map.iter().all(|l| l.len() == width)
	}
}

fn count_day3(map: &Map, right_step: usize, down_step: usize) -> u64 {
	(0..map.height()).step_by(down_step).fold(0, |acc, y| {
		let x = match y {
			0 => 0,
			_ => (y / down_step) * right_step,
		};
		match map.lookup(x, y) {
			Some(Terrain::Tree) => acc + 1,
			_ => acc,
		}
	})
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = Vec::new();

	stdin
		.lock()
		.read_to_end(&mut buffer)
		.expect("Failed to read from stdin");
	let map = parse_map(&buffer)
		.map(|(_, m)| m)
		.expect("Failed to parse input as map");
	assert!(map.validate());

	let answer_slope1 = count_day3(&map, 1, 1);
	let answer_slope3 = count_day3(&map, 3, 1);
	let answer_slope5 = count_day3(&map, 5, 1);
	let answer_slope7 = count_day3(&map, 7, 1);
	let answer_down2 = count_day3(&map, 1, 2);
	let answer = answer_slope1
		* answer_slope3
		* answer_slope5
		* answer_slope7
		* answer_down2;

	println!("Answer slope1: {}", answer_slope1);
	println!("Answer slope3: {}", answer_slope3);
	println!("Answer slope5: {}", answer_slope5);
	println!("Answer slope7: {}", answer_slope7);
	println!("Answer down2: {}", answer_down2);
	println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_terrain_open() {
		let (rest, r) = parse_terrain(b".").unwrap();
		assert_eq!(0, rest.len());
		assert_eq!(Terrain::Open, r);
	}

	#[test]
	fn test_parse_terrain_tree() {
		let (rest, r) = parse_terrain(b"#").unwrap();
		assert_eq!(0, rest.len());
		assert_eq!(Terrain::Tree, r);
	}

	#[test]
	fn test_parse_map_line() {
		let (rest, r) = parse_map_line(b".#.").unwrap();
		assert_eq!(0, rest.len());
		assert_eq!(vec![Terrain::Open, Terrain::Tree, Terrain::Open], r);
	}

	#[test]
	fn test_parse_map() {
		let (rest, Map::Map(r)) = parse_map(b".\n#\n.").unwrap();
		assert_eq!(0, rest.len());
		assert_eq!(
			vec![
				vec![Terrain::Open],
				vec![Terrain::Tree],
				vec![Terrain::Open]
			],
			r
		);
	}

	#[test]
	fn test_count_day3_open() {
		let map = Map::Map(vec![vec![Terrain::Open]]);
		let count = count_day3(&map, 3, 1);
		assert_eq!(0, count);
	}

	#[test]
	fn test_count_day3_tree() {
		let map = Map::Map(vec![vec![Terrain::Tree]]);
		let count = count_day3(&map, 3, 1);
		assert_eq!(1, count);
	}

	#[test]
	fn test_count_day3_tree_open_tree() {
		let map = Map::Map(vec![
			vec![Terrain::Tree],
			vec![Terrain::Open],
			vec![Terrain::Tree],
		]);
		let count = count_day3(&map, 3, 1);
		assert_eq!(2, count);
	}

	#[test]
	fn test_count_day3_right1() {
		let map = Map::Map(vec![
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Tree, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Tree, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Tree],
			vec![Terrain::Tree, Terrain::Open, Terrain::Open, Terrain::Open],
		]);
		let count = count_day3(&map, 1, 1);
		assert_eq!(4, count);
	}

	#[test]
	fn test_count_day3_down2() {
		let map = Map::Map(vec![
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Tree, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Tree, Terrain::Open],
		]);
		let count = count_day3(&map, 1, 2);
		assert_eq!(2, count);
	}

	#[test]
	fn test_count_day3_wrap5() {
		let map = Map::Map(vec![
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Tree, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Tree, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Tree],
		]);
		let count = count_day3(&map, 5, 1);
		assert_eq!(3, count);
	}

	#[test]
	fn test_count_day3() {
		let map = Map::Map(vec![
			vec![Terrain::Tree, Terrain::Open, Terrain::Open, Terrain::Open],
			vec![Terrain::Open, Terrain::Open, Terrain::Open, Terrain::Tree],
		]);
		let count = count_day3(&map, 3, 1);
		assert_eq!(2, count);
	}
}
