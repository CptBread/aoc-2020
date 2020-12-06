use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn solve() {
	let file = File::open("data/day6.txt").unwrap();
	let read = BufReader::new(file);

	let mut count = 0;
	let mut count_b = 0;
	let mut ans = [false; 26];
	let mut ans_b = [0; 26];
	let mut group = 0;
	for l in read.lines().map(|l| l.unwrap()) {
		if l == "" {
			for i in 0..ans.len() {
				count_b += (ans_b[i] == group) as usize;
			}
			ans = [false; 26];
			ans_b = [0; 26];
			group = 0;
		}
		else
		{
			for c in l.bytes() {
				if c.is_ascii_lowercase() {
					let idx = (c - b'a') as usize;
					if !ans[idx] {
						ans[idx] = true;
						count += 1;
					}
					if ans_b[idx] == group {
						ans_b[idx] += 1;
					}
				} else {
					panic!("We assumed only lower case ascii characters... That was wrong... Got: {}", c as char);
				}
			}
			group += 1;
		}
	}
	if group > 0 {
		for i in 0..ans.len() {
			count_b += (ans_b[i] == group) as usize;
		}
	}
	println!("{} {}", count, count_b);
	assert_eq!(count, 6742);
	assert_eq!(count_b, 3447);
}