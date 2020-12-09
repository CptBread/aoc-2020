use std::fs::File;
use std::io::{BufReader, prelude::*};
use crate::utils::split_once;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day2.txt").unwrap();
	let read = BufReader::new(f);
	let mut valid_a = 0;
	let mut valid_b = 0;
	for l in read.lines().map(|l| l.unwrap()) {
		let mut it = l.split(' ');
		let (min, max) = split_once(it.next().unwrap(), "-").unwrap();
		let min = min.parse::<usize>().unwrap();
		let max = max.parse::<usize>().unwrap();
		let c = it.next().unwrap().chars().next().unwrap();
		let pass = it.next().unwrap();
		let count = pass.chars().fold(0, |count, b| if b == c as char {count + 1} else {count} );
		if count >= min && count <= max {
			valid_a += 1;
		}
		if pass.chars().skip(min - 1).next().map_or(false, |b| b == c as char) != 
			pass.chars().skip(max - 1).next().map_or(false, |b| b == c as char)
		{
			// println!("{:?} {:?} c: {} pass:{}", pass.chars().skip(min - 1).next(), pass.chars().skip(max - 1).next(), c, pass);
			valid_b += 1;
		}
	}
	println!("A:{} B:{}", valid_a, valid_b);
	assert_eq!(valid_a, 600);
	assert_eq!(valid_b, 245);
}
