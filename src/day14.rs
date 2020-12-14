use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;
use crate::utils::split_once;

pub fn solve() {
	let file = File::open("data/day14.txt").unwrap();
	let read = BufReader::new(file);
	let mut mem = HashMap::new();
	let mut masks = (0, 0);
	let mut sum = 0;
	for l in read.lines().map(Result::unwrap) {
		if let Some(s) = l.strip_prefix("mask = ") {
			masks = (0, 0);
			for c in s.chars() {
				masks.0 = (masks.0 << 1) + (c != '0') as usize;
				masks.1 = (masks.1 << 1) + (c == '1') as usize;
			}
			// println!("Masks 0:{:b} 1:{:b}", masks.0, masks.1);
		} else {
			let s = l.strip_prefix("mem[").unwrap();
			let (addrs, s) = split_once(s, "]").unwrap();
			let addrs = addrs.parse::<usize>().unwrap();
			let mut val = s.strip_prefix(" = ").unwrap().parse::<usize>().unwrap();
			// let org = val;
			// println!("{}:{:b}", val, val);
			val &= masks.0;
			// println!("{}:{:b}", val, val);
			val |= masks.1;
			// println!("{}:{:b}", val, val);
			// println!("adding {}:{:b} (org {}:{:b})", val, val, org, org);
			sum += val as i128;
			if let Some(v) = mem.insert(addrs, val) {
				sum -= v as i128;
			}
		}
	}

	println!("{}", sum);
	assert_eq!(sum, 17481577045893);

	let file = File::open("data/day14.txt").unwrap();
	let read = BufReader::new(file);
	let mut mem = HashMap::new();
	let mut masks = (0, 0);
	let mut mask = 0;
	let mut sum = 0;
	for l in read.lines().map(Result::unwrap) {
		if let Some(s) = l.strip_prefix("mask = ") {
			masks = (0, 0);
			for c in s.chars() {
				masks.0 = (masks.0 << 1) + (c == '0') as usize;
				masks.1 = (masks.1 << 1) + (c == '1') as usize;
			}
			mask = masks.0 | masks.1;
			// println!("Masks 0:{:b} 1:{:b} comb {:b}", masks.0, masks.1, mask);
		} else {
			let s = l.strip_prefix("mem[").unwrap();
			let (addrs, s) = split_once(s, "]").unwrap();
			let addrs = addrs.parse::<usize>().unwrap();
			let addrs = addrs | masks.1;
			let val = s.strip_prefix(" = ").unwrap().parse::<usize>().unwrap();
			set_mems(35, val, addrs, mask, &mut mem, &mut sum);
		}
	}
	println!("{}", sum);
	assert_eq!(sum, 4160009892257);
}

fn set_mems(bit: i8, val: usize, addrs: usize, mask: usize, mem: &mut HashMap<usize, usize>, sum: &mut usize) {
	if bit >= 0 {
		for bit in (0..=bit).rev() {
			let at = 1 << bit;
			if mask & at == 0 {
				let addrs = addrs | at;
				set_mems(bit - 1, val, addrs, mask, mem, sum);
				let addrs = addrs & (!at);
				set_mems(bit - 1, val, addrs, mask, mem, sum);
				return;
			}
		}
	}
	*sum += val;
	if let Some(v) = mem.insert(addrs, val) {
		*sum -= v
	}
}