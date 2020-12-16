use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::ops::RangeInclusive;
use std::collections::HashMap;
use intbits::Bits;
use crate::utils::split_once;

fn parse_range(s: &str) -> RangeInclusive<usize> {
	let mut it = s.split("-").map(|s| s.parse::<usize>().unwrap());
	it.next().unwrap()..=it.next().unwrap()
}

fn ranges_contains(r: &[RangeInclusive<usize>; 2], v: usize) -> bool {
	r[0].contains(&v) || r[1].contains(&v)
}

fn single_bit(data: usize, size: usize) -> Option<usize> {
	let mut found = None;
	for idx in 0..size {
		if data.bit(idx) {
			if found.is_some() {
				return None;
			} else {
				found = Some(idx);
			}
		}
	}
	found
}

pub fn solve() {
	let file = File::open("data/day16.txt").unwrap();
	let read = BufReader::new(file);
	let mut lines = read.lines().map(Result::unwrap);
	// let mut rules = HashMap::new();
	let mut categories = Vec::new();
	let mut rules = Vec::new();
	while let Some(l) = lines.next() {
		if l == "" {
			break;
		}
		let (cat, l) = split_once(&l, ": ").unwrap();
		let (low, high) = split_once(&l, " or ").unwrap();
		let low = parse_range(low);
		let high = parse_range(high);
		rules.push([low, high]);
		categories.push(cat.to_string());
	}
	// println!("{:?}", rules);
	lines.next();
	let ours: Vec<_> = lines.next().unwrap().split(',').map(|s| s.parse::<usize>().unwrap()).collect();
	// println!("{:?}", mine);
	lines.next();
	lines.next();
	let others: Vec<Vec<_>> = lines.map(|s| s.split(',').map(|s| s.parse::<usize>().unwrap()).collect()).collect();
	// println!("{:?}", others);
	let mut invalid = 0;
	let mut possible = Vec::new();
	let cats = categories.len();
	possible.resize(cats, !0 ^ (!0 << cats));
	let others_it = others.iter().filter(|v| v.iter().all(|v| {
		if !rules.iter().any(|a| ranges_contains(a, *v)) {
			invalid += *v;
			false
		} else {true}
	}));

	for ticket in others_it {
		for (idx, v) in ticket.iter().cloned().enumerate() {
			for (r_idx, r) in rules.iter().enumerate() {
				if possible[idx].bit(r_idx) && !ranges_contains(r, v) {
					possible[idx].set_bit(r_idx, false); 
				}
			}
		}
	}
	let mut to_check: Vec<usize> = (0..cats).collect();
	let mut known = HashMap::new();
	loop {
		let size = to_check.len();
		to_check.retain(|idx| {
			if let Some(bit_idx) = single_bit(possible[*idx], cats) {
				// println!("{}: {}", categories[bit_idx], ours[*idx]);
				possible.iter_mut().enumerate().for_each(|(n, p)| {
					if n != *idx {
						p.set_bit(bit_idx, false);
					}
				});
				known.insert(&categories[bit_idx], *idx);
				false
			} else {true}
		});
		if size == to_check.len() || to_check.len() == 0 {
			break;
		}
	}

	println!("{}", invalid);
	assert_eq!(invalid, 30869);

	let mut res = 1;
	for (k, v) in known.iter() {
		if k.starts_with("departure") {
			res *= ours[*v];
		}
	}
	// println!("{:?}", known);
	println!("{}", res);
	assert_eq!(res, 4381476149273);
}