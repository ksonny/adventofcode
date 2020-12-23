use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{digit1, newline},
	combinator::{map, map_res},
	error,
	multi::many1,
	sequence::{pair, preceded, terminated},
	Err::Error,
	IResult,
};
use std::{io, io::Read, str};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Immediate {
	Pos(u64),
	Neg(u64),
}

#[derive(Debug, Clone, PartialEq)]
enum Instr {
	Acc(Immediate),
	Jmp(Immediate),
	Nop(Immediate),
}

#[derive(Debug)]
struct VirtualMachine {
	pc: usize,
	acc: i64,
}

impl VirtualMachine {
	fn new() -> Self {
		VirtualMachine { pc: 0, acc: 0 }
	}

	fn step(&mut self, p: &Vec<Instr>) -> Option<i64> {
		match p.get(self.pc) {
			Some(Instr::Acc(Immediate::Pos(i))) => {
				self.pc += 1;
				self.acc += *i as i64;
				None
			}
			Some(Instr::Acc(Immediate::Neg(i))) => {
				self.pc += 1;
				self.acc -= *i as i64;
				None
			}
			Some(Instr::Jmp(Immediate::Pos(i))) => {
				self.pc += *i as usize;
				None
			}
			Some(Instr::Jmp(Immediate::Neg(i))) => {
				self.pc -= *i as usize;
				None
			}
			Some(Instr::Nop(_)) => {
				self.pc += 1;
				None
			}
			None if self.pc == p.len() => Some(self.acc),
			None => panic!("Outside of bounds"),
		}
	}
}

fn immediate(i: &[u8]) -> IResult<&[u8], Immediate> {
	map(
		pair(
			alt((tag("+"), tag("-"))),
			map_res(map_res(digit1, str::from_utf8), str::parse),
		),
		|(s, n)| match s {
			b"+" => Immediate::Pos(n),
			b"-" => Immediate::Neg(n),
			_ => panic!("Unknown sign"),
		},
	)(i)
}

fn instruction(i: &[u8]) -> IResult<&[u8], Instr> {
	terminated(
		alt((
			preceded(tag("acc "), map(immediate, |n| Instr::Acc(n))),
			preceded(tag("jmp "), map(immediate, |n| Instr::Jmp(n))),
			preceded(tag("nop "), map(immediate, |n| Instr::Nop(n))),
		)),
		newline,
	)(i)
}

fn program(i: &[u8]) -> IResult<&[u8], Vec<Instr>> {
	many1(instruction)(i)
}

fn find_answer_part1(p: &Vec<Instr>) -> i64 {
	let mut vm = VirtualMachine::new();
	let mut tr = vec![0; p.len()];

	loop {
		match tr.get_mut(vm.pc) {
			Some(n) if *n > 0 => return vm.acc,
			Some(n) => *n += 1,
			None => panic!(),
		}
		vm.step(p);
	}
}

fn find_answer_part2(p: &Vec<Instr>) -> Option<i64> {
	let mut vm = VirtualMachine::new();
	let mut tr = vec![None; p.len()];

	loop {
		match tr.get_mut(vm.pc) {
			Some(Some(_)) => break,
			Some(r @ None) => *r = Some((vm.pc, vm.acc)),
			None => panic!(),
		}
		vm.step(p);
	}

	let candidates = tr.iter().zip(p.iter()).filter_map(|r| match r {
		(Some((pc, acc)), Instr::Nop(Immediate::Pos(i))) => {
			Some((pc + *i as usize, *acc))
		}
		(Some((pc, acc)), Instr::Nop(Immediate::Neg(i))) => {
			Some((pc - *i as usize, *acc))
		}
		(Some((pc, acc)), Instr::Jmp(_)) => Some((pc + 1, *acc)),
		_ => None,
	});

	for (pc, acc) in candidates {
		vm.pc = pc;
		vm.acc = acc;
		loop {
			if tr.get(vm.pc).map_or(false, |r| r.is_some()) {
				break;
			}
			if let Some(r) = vm.step(p) {
				return Some(r);
			}
		}
	}
	None
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = Vec::new();
	stdin
		.lock()
		.read_to_end(&mut buffer)
		.expect("Failed to read from stdin");
	let (rest, p) = match program(&buffer) {
		Err(Error(error::Error { input, code: _ })) => panic!(
			"Error occured processing input: {}",
			str::from_utf8(input).unwrap()
		),
		Ok(x) => x,
		e => panic!("Other error: {:?}", e),
	};
	if rest.len() > 0 {
		panic!(
			"Not all input used: {}",
			str::from_utf8(rest).unwrap_or("[Invalid utf8]")
		);
	}

	let answer_part1 = find_answer_part1(&p);
	let answer_part2 = find_answer_part2(&p).expect("No answer found for part2");

	println!("Answer part1: {}", answer_part1);
	println!("Answer part2: {}", answer_part2);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_immediate() {
		let (rest, r) = immediate(b"+10").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Immediate::Pos(10), r);
	}

	#[test]
	fn test_parse_immediate_negative() {
		let (rest, r) = immediate(b"-10").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Immediate::Neg(10), r);
	}

	#[test]
	fn test_parse_instruction_acc() {
		let (rest, r) =
			instruction(b"acc +10\n").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Instr::Acc(Immediate::Pos(10)), r);
	}

	#[test]
	fn test_parse_instruction_jmp() {
		let (rest, r) =
			instruction(b"jmp +10\n").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Instr::Jmp(Immediate::Pos(10)), r);
	}

	#[test]
	fn test_parse_instruction_nop() {
		let (rest, r) =
			instruction(b"nop +10\n").expect("Failed to parse input");
		assert_eq!(b"", rest, "Not all input consumed");
		assert_eq!(Instr::Nop(Immediate::Pos(10)), r);
	}

	#[test]
	fn test_vm_step_acc() {
		let mut vm = VirtualMachine::new();
		vm.step(&vec![Instr::Acc(Immediate::Pos(1))]);
		assert_eq!(vm.pc, 1);
		assert_eq!(vm.acc, 1);
	}

	#[test]
	fn test_vm_step_jmp_pos() {
		let mut vm = VirtualMachine::new();
		vm.step(&vec![Instr::Jmp(Immediate::Pos(1))]);
		assert_eq!(vm.pc, 1);
		assert_eq!(vm.acc, 0);
	}
	#[test]
	fn test_vm_step_jmp_zero() {
		let mut vm = VirtualMachine::new();
		vm.step(&vec![Instr::Jmp(Immediate::Pos(0))]);
		assert_eq!(vm.pc, 0);
		assert_eq!(vm.acc, 0);
	}

	#[test]
	fn test_vm_step_jmp_neg() {
		let mut vm = VirtualMachine::new();
		vm.pc = 1;
		vm.step(&vec![
			Instr::Nop(Immediate::Pos(0)),
			Instr::Jmp(Immediate::Neg(1)),
		]);
		assert_eq!(vm.pc, 0);
		assert_eq!(vm.acc, 0);
	}
}
