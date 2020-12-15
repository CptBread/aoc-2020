use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;

const NUMS: &[usize] = &[
	0,5,4,1,10,14,7
];

pub fn solve() {
	let mut said_at: HashMap<usize, usize> = HashMap::new();
	let mut last = NUMS[0];
	let mut at = 1;
	for n in &NUMS[at..] {
		said_at.insert(last, at);
		last = *n;
		at += 1;
		// println!("{}", last);
	}
	for _ in at..2020 {
		let last_at = said_at.insert(last, at).unwrap_or(at);
		let new = at - last_at;
		// println!("{}", new);
		last = new;
		at += 1;
	}
	println!("{}", last);
	assert_eq!(last, 203);

	for _ in at..30000000 {
		let last_at = said_at.insert(last, at).unwrap_or(at);
		let new = at - last_at;
		// println!("{}", new);
		last = new;
		at += 1;
	}
	println!("{}", last);
	assert_eq!(last, 9007186);
}