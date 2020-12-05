use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn solve() {
	let f = File::open("data/day5.txt").unwrap();
	let read = BufReader::new(f);

	let mut ids = Vec::new();
	let mut high = 0;
	for l in read.lines().map(|l| l.unwrap()) {
		let (r, c) = l.bytes().fold((0, 0), |(r, c), b| {
			match b {
				b'B' => ((r << 1) + 1, c),
				b'F' => (r << 1, c),
				b'R' => (r, (c << 1) + 1),
				b'L' => (r, c << 1),
				_ => {
					println!("Unknown: {}", b as char);
					(r, c)
				},
			}
		});
		let id = r * 8 + c;
		high = high.max(r * 8 + c);
		ids.push(id);
		// println!("{} {} {}", r, c, r * 8 + c);
	}
	println!("{}", high);
	assert_eq!(high, 835);

	ids.sort();
	let mut last = 0;
	for id in ids {
		// println!("{}", id);
		if last + 2 == id {
			println!("{}", id - 1);
			assert_eq!(id - 1, 649);
			break;
		}
		last = id;
	}
}