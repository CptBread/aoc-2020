use std::fs::File;
use std::io::{BufReader, prelude::*};
use crate::utils::UtilRead;

#[allow(dead_code)]
pub fn solve() {
	let f = File::open("data/day2.txt").unwrap();
	let mut read = BufReader::new(f);
	let mut valid_a = 0;
	let mut valid_b = 0;
	let _ = || -> Option<()> {
		loop {
			let min = read.parse_until::<usize>(b'-')?;
			let max = read.parse_until::<usize>(b' ')?;
			let c = read.parse_until::<char>(b':')?;
			read.consume(1);
			let pass = read.read_string_until(b'\n');

			// println!("{}-{} {}: {}", min, max, c, pass);
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
	}();
	println!("A:{} B:{}", valid_a, valid_b);
}
