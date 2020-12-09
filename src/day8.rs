use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Instruction {
	Nop(i32),
	Acc(i32),
	Jmp(i32),
}

#[derive(Debug, Clone)]
struct Computer {
	pub idx: usize,
	pub acc: i32,
	pub code: Vec<Instruction>,
}

impl Computer {
	pub fn do_one(&mut self) -> Option<usize> {
		self.do_inst(*self.code.get(self.idx)?);
		Some(self.idx)
	}

	pub fn do_one_what(&mut self) -> Option<(usize, Instruction)> {
		let inst = self.code.get(self.idx)?.clone();
		self.do_inst(inst);
		Some((self.idx, inst))
	}

	fn do_inst(&mut self, inst: Instruction) {
		self.idx = match inst {
			Nop(_) => {self.idx + 1},
			Acc(v) => {self.acc += v; self.idx + 1},
			Jmp(v) => {self.idx.wrapping_add((v - 1) as usize)}
		}
	}
}

// Not using nightly so split once coudln't be used...
fn split_once<'a>(s: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
	let mut it = s.splitn(2, delim);
	Some((it.next()?, it.next()?))
}

use Instruction::*;
pub fn solve() {
	let file = File::open("data/day8.txt").unwrap();
	let read = BufReader::new(file);

	let mut code = Vec::new();
	for l in read.lines().map(|l| l.unwrap()) {
		let (inst, arg) = split_once(&l[..], " ").expect("Invalid instruction!");
		let num = if arg.chars().next() == Some('-') { arg.parse() } else { arg[1..].parse() }.expect("Invalid argument!");
		match inst {
			"nop" => {code.push(Nop(num))},
			"acc" => {code.push(Acc(num))},
			"jmp" => {code.push(Jmp(num))},
			_ => {panic!("Invalid argument! {} {}", inst, arg)}
		}
	}
	let mut comp = Computer{
		idx: 0,
		acc: 0,
		code,
	};
	let mut found = HashSet::new();
	found.insert(0);
	let mut order = Vec::new();
	while let Some(idx) = comp.do_one() {
		if !found.insert(idx) {
			println!("{}", comp.acc);
			assert_eq!(1337, comp.acc);
			break;
		}
		order.push(idx);
	}

	comp.idx = 0;
	comp.acc = 0;
	for idx in order.iter().rev().cloned() {
		if let Some(new) = match comp.code[idx] {
			Nop(v) => {Some(Jmp(v))},
			Jmp(v) => {Some(Nop(v))},
			_ => None,
		} {
			let mut comp = comp.clone();
			comp.code[idx] = new;
			if let Some(acc) = check_halts(comp) {
				println!("{}", acc);
			assert_eq!(1358, acc);
			break;
			}
		}
	}
}

fn check_halts(mut comp: Computer) -> Option<i32> {
	let mut found = HashSet::new();
	found.insert(comp.idx);
	while let Some(idx) = comp.do_one() {
		if !found.insert(idx) {
			return None;
		}
	}
	Some(comp.acc)
}