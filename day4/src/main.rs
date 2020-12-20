use nom::{
	branch::alt,
	bytes::complete::{tag, take_while, take_while_m_n},
	character::{
		complete::{alphanumeric1, newline, none_of, one_of},
		is_digit, is_hex_digit,
	},
	combinator::{map, map_res, recognize},
	multi::{many1, separated_list1},
	sequence::{pair, terminated},
	IResult,
};
use std::{
	io::{self, Read},
	str,
};

#[derive(Debug, PartialEq)]
enum Field<'a> {
	Field(&'a [u8], &'a [u8]),
}

#[derive(Debug, PartialEq)]
enum Passport<'a> {
	Passport(Vec<Field<'a>>),
}

#[derive(Debug, PartialEq)]
enum Unit {
	Cm,
	In,
}

fn parse_field<'a>(i: &'a [u8]) -> IResult<&'a [u8], Field<'a>> {
	map(
		pair(
			terminated(alphanumeric1, tag(":")),
			recognize(many1(none_of(" \t\n"))),
		),
		|(k, v)| Field::Field(k, v),
	)(i)
}

fn parse_passport<'a>(i: &'a [u8]) -> IResult<&'a [u8], Passport<'a>> {
	map(
		separated_list1(alt((one_of(" \t"), newline)), parse_field),
		|fs| Passport::Passport(fs),
	)(i)
}

fn parse_passports<'a>(i: &'a [u8]) -> IResult<&'a [u8], Vec<Passport<'a>>> {
	separated_list1(pair(newline, newline), parse_passport)(i)
}

fn parse_year(i: &[u8]) -> IResult<&[u8], u64> {
	map_res(
		map_res(take_while_m_n(4, 4, is_digit), |r| str::from_utf8(r)),
		str::parse,
	)(i)
}

fn parse_length(i: &[u8]) -> IResult<&[u8], (u64, Unit)> {
	pair(
		map_res(
			map_res(recognize(take_while(is_digit)), |r| str::from_utf8(r)),
			str::parse,
		),
		alt((map(tag("cm"), |_| Unit::Cm), map(tag("in"), |_| Unit::In))),
	)(i)
}

fn parse_color(i: &[u8]) -> IResult<&[u8], &str> {
	map_res(
		pair(tag("#"), take_while_m_n(6, 6, is_hex_digit)),
		|(_, r)| str::from_utf8(r),
	)(i)
}

fn parse_digit_count(i: &[u8]) -> IResult<&[u8], usize> {
	map_res(take_while(is_digit), |r| str::from_utf8(r).map(|s| s.len()))(i)
}

struct Validation {
	has_byr: bool,
	has_iyr: bool,
	has_eyr: bool,
	has_hgt: bool,
	has_hcl: bool,
	has_ecl: bool,
	has_pid: bool,
}

impl Passport<'_> {
	fn valid_part1(&self) -> bool {
		let Passport::Passport(fields) = self;
		let mut v = Validation {
			has_byr: false,
			has_iyr: false,
			has_eyr: false,
			has_hgt: false,
			has_hcl: false,
			has_ecl: false,
			has_pid: false,
		};
		for field in fields {
			let Field::Field(k, _) = field;
			match str::from_utf8(k) {
				Ok("byr") => v.has_byr = true,
				Ok("iyr") => v.has_iyr = true,
				Ok("eyr") => v.has_eyr = true,
				Ok("hgt") => v.has_hgt = true,
				Ok("hcl") => v.has_hcl = true,
				Ok("ecl") => v.has_ecl = true,
				Ok("pid") => v.has_pid = true,
				_ => {}
			};
		}
		v.has_byr
			&& v.has_iyr && v.has_eyr
			&& v.has_hgt && v.has_hcl
			&& v.has_ecl && v.has_pid
	}

	fn valid_part2(&self) -> bool {
		let Passport::Passport(fields) = self;
		let mut val = Validation {
			has_byr: false,
			has_iyr: false,
			has_eyr: false,
			has_hgt: false,
			has_hcl: false,
			has_ecl: false,
			has_pid: false,
		};
		for field in fields {
			let Field::Field(k, v) = field;
			match str::from_utf8(k) {
				Ok("byr") => {
					val.has_byr = parse_year(v)
						.map_or(false, |(_, year)| year >= 1920 && year <= 2002)
				}
				Ok("iyr") => {
					val.has_iyr = parse_year(v)
						.map_or(false, |(_, year)| year >= 2010 && year <= 2020)
				}
				Ok("eyr") => {
					val.has_eyr = parse_year(v)
						.map_or(false, |(_, year)| year >= 2020 && year <= 2030)
				}
				Ok("hgt") => {
					val.has_hgt =
						parse_length(v).map_or(false, |(_, (l, u))| match u {
							Unit::Cm => l >= 150 && l <= 193,
							Unit::In => l >= 59 && l <= 76,
						})
				}
				Ok("hcl") => val.has_hcl = parse_color(v).is_ok(),
				Ok("ecl") => {
					val.has_ecl = match str::from_utf8(v) {
						Ok("amb") | Ok("blu") | Ok("brn") | Ok("gry")
						| Ok("grn") | Ok("hzl") | Ok("oth") => true,
						_ => false,
					}
				}
				Ok("pid") => {
					val.has_pid =
						parse_digit_count(v).map_or(false, |(_, c)| c == 9)
				}
				_ => {}
			};
		}
		val.has_byr
			&& val.has_iyr
			&& val.has_eyr
			&& val.has_hgt
			&& val.has_hcl
			&& val.has_ecl
			&& val.has_pid
	}
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = Vec::new();
	stdin
		.lock()
		.read_to_end(&mut buffer)
		.expect("Failed to read from stdin");

	let passports = parse_passports(&buffer)
		.map(|(_, ps)| ps)
		.expect("Failed to parse passports");
	let valid_count_part1 =
		passports
			.iter()
			.fold(0, |acc, p| if p.valid_part1() { acc + 1 } else { acc });
	let valid_count_part2 =
		passports
			.iter()
			.fold(0, |acc, p| if p.valid_part2() { acc + 1 } else { acc });

	println!("Answer part1: {}", valid_count_part1);
	println!("Answer part2: {}", valid_count_part2);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_field() {
		let (rest, r) = parse_field(b"key:value").unwrap();
		assert_eq!(0, rest.len());
		assert_eq!(Field::Field(b"key", b"value"), r);
	}

	#[test]
	fn test_parse_passport_color() {
		let input = b"hcl:#602927";
		let (rest, Passport::Passport(r)) = parse_passport(input).unwrap();
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!(vec![Field::Field(b"hcl", b"#602927"),], r);
	}

	#[test]
	fn test_parse_passport_multi_space() {
		let (rest, Passport::Passport(r)) = parse_passport(b"a:1 b:2").unwrap();
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!(vec![Field::Field(b"a", b"1"), Field::Field(b"b", b"2")], r);
	}

	#[test]
	fn test_parse_passport_multi_newline() {
		let (rest, Passport::Passport(r)) =
			parse_passport(b"a:1\nb:2").unwrap();
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!(vec![Field::Field(b"a", b"1"), Field::Field(b"b", b"2")], r);
	}

	#[test]
	fn test_parse_passport_delimeted() {
		let (rest, Passport::Passport(r)) =
			parse_passport(b"a:1\n\nb:2").unwrap();
		assert_eq!(b"\n\nb:2", rest, "Expected partial parse");
		assert_eq!(vec![Field::Field(b"a", b"1")], r);
	}

	#[test]
	fn test_parse_passports() {
		let (rest, ps) = parse_passports(b"a:1\n\nb:2").unwrap();
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);

		let Passport::Passport(a) =
			ps.get(0).expect("Failed to parse passport a");
		let Passport::Passport(b) =
			ps.get(1).expect("Failed to parse passport b");

		assert_eq!(vec![Field::Field(b"a", b"1")], *a);
		assert_eq!(vec![Field::Field(b"b", b"2")], *b);
	}

	#[test]
	fn test_parse_passports_with_newline() {
		let (rest, ps) = parse_passports(b"a:1\na:2\n\nb:1").unwrap();
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);

		let Passport::Passport(a) =
			ps.get(0).expect("Failed to parse passport a");
		let Passport::Passport(b) =
			ps.get(1).expect("Failed to parse passport b");

		assert_eq!(
			vec![Field::Field(b"a", b"1"), Field::Field(b"a", b"2")],
			*a
		);
		assert_eq!(vec![Field::Field(b"b", b"1")], *b);
	}

	#[test]
	fn test_parse_year() {
		let (rest, r) = parse_year(b"1990").expect("Failed to parse input");
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!(1990, r);
	}

	#[test]
	fn test_parse_length_cm() {
		let (rest, r) = parse_length(b"199cm").expect("Failed to parse input");
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!((199, Unit::Cm), r);
	}

	#[test]
	fn test_parse_length_in() {
		let (rest, r) = parse_length(b"99in").expect("Failed to parse input");
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!((99, Unit::In), r);
	}

	#[test]
	fn test_parse_color() {
		let (rest, r) = parse_color(b"#aabbcc").expect("Failed to parse input");
		assert_eq!(
			b"",
			rest,
			"Not all input consumed, rest: {}",
			str::from_utf8(rest).unwrap()
		);
		assert_eq!("aabbcc", r);
	}

	#[test]
	fn test_passport_valid_part1_empty_invalid() {
		let a = Passport::Passport(vec![]);
		assert!(!a.valid_part1());
	}

	#[test]
	fn test_passport_valid_part1_valid() {
		let a = Passport::Passport(vec![
			Field::Field(b"byr", b"1"),
			Field::Field(b"iyr", b"2"),
			Field::Field(b"eyr", b"3"),
			Field::Field(b"hgt", b"4"),
			Field::Field(b"hcl", b"5"),
			Field::Field(b"ecl", b"6"),
			Field::Field(b"pid", b"7"),
		]);
		assert!(a.valid_part1());
	}

	#[test]
	fn test_passport_valid_part1_cid_valid() {
		let a = Passport::Passport(vec![
			Field::Field(b"byr", b"1"),
			Field::Field(b"iyr", b"2"),
			Field::Field(b"eyr", b"3"),
			Field::Field(b"hgt", b"4"),
			Field::Field(b"hcl", b"5"),
			Field::Field(b"ecl", b"6"),
			Field::Field(b"pid", b"7"),
			Field::Field(b"cid", b"8"),
		]);
		assert!(a.valid_part1());
	}

	#[test]
	fn test_passport_valid_part2_valid() {
		let a = Passport::Passport(vec![
			Field::Field(b"byr", b"1980"),
			Field::Field(b"iyr", b"2012"),
			Field::Field(b"eyr", b"2030"),
			Field::Field(b"hgt", b"74in"),
			Field::Field(b"hcl", b"#623a2f"),
			Field::Field(b"ecl", b"grn"),
			Field::Field(b"pid", b"087499704"),
		]);
		assert!(a.valid_part2());
	}
}
