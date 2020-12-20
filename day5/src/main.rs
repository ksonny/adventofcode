use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::newline,
	combinator::map,
	multi::{count, separated_list1},
	sequence::pair,
	IResult,
};
use std::io::{self, Read};

#[derive(Debug, PartialEq)]
enum Seat {
	Seat(u32),
}

impl Seat {
	fn id(&self) -> u32 {
		let &Seat::Seat(seat_id) = self;
		seat_id
	}
}

fn parse_fb(i: &[u8]) -> IResult<&[u8], bool> {
	alt((map(tag("F"), |_| false), map(tag("B"), |_| true)))(i)
}

fn parse_lr(i: &[u8]) -> IResult<&[u8], bool> {
	alt((map(tag("L"), |_| false), map(tag("R"), |_| true)))(i)
}

fn parse_seat(i: &[u8]) -> IResult<&[u8], Seat> {
	map(pair(count(parse_fb, 7), count(parse_lr, 3)), |(fb, lr)| {
		let iter = fb.iter().chain(lr.iter());
		let seat_id = iter.fold(0, |acc, &b| acc << 1 | b as u32);
		Seat::Seat(seat_id)
	})(i)
}

fn parse_seats(i: &[u8]) -> IResult<&[u8], Vec<Seat>> {
	separated_list1(newline, parse_seat)(i)
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = Vec::new();
	stdin
		.lock()
		.read_to_end(&mut buffer)
		.expect("Failed to read from stdin");

	let mut seats = parse_seats(&buffer)
		.map(|(_, ss)| ss)
		.expect("Failed to parse seats");

	seats.sort_by(|a, b| a.id().cmp(&b.id()));

	let max_seat = seats.last();

	let my_seat = seats
		.iter()
		.zip(seats.iter().skip(1))
		.find(|r| match r {
			&(a, b) if a.id() + 1 != b.id() => true,
			_ => false,
		});

	println!("Answer part1: {:?}", max_seat);
	println!("Answer part2: {:?}", my_seat);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_fb_f() {
		let (rest, r) = parse_fb(b"F").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(false, r);
	}

	#[test]
	fn test_parse_fb_b() {
		let (rest, r) = parse_fb(b"B").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(true, r);
	}

	#[test]
	fn test_parse_lr_l() {
		let (rest, r) = parse_lr(b"L").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(false, r);
	}

	#[test]
	fn test_parse_lr_r() {
		let (rest, r) = parse_lr(b"R").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(true, r);
	}

	#[test]
	fn test_parse_seat_row_0() {
		let (rest, r) =
			parse_seat(b"FFFFFFFLLL").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Seat::Seat(0_u32), r);
	}

	#[test]
	fn test_parse_seat_row_127() {
		let (rest, r) =
			parse_seat(b"BBBBBBBLLL").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Seat::Seat(127_u32 << 3), r);
	}

	#[test]
	fn test_parse_seat_column_1() {
		let (rest, r) =
			parse_seat(b"FFFFFFFLLR").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Seat::Seat(1_u32), r);
	}

	#[test]
	fn test_parse_seat_column_7() {
		let (rest, r) =
			parse_seat(b"FFFFFFFRRR").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Seat::Seat(7_u32), r);
	}
}
