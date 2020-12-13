use std::fs::File;
use std::io::{BufReader, prelude::*};

fn gcd(mut x: usize, mut y: usize) -> usize {
	while y != 0 {
		let t = y;
		y = x % y;
		x = t;
	}
	x
}

fn lcm(l:usize, r:usize) -> usize {
	l * (r / gcd(l, r))
}

pub fn solve() {
	let file = File::open("data/day13.txt").unwrap();
	let read = BufReader::new(file);
	let mut read = read.lines().map(Result::unwrap);
	let start = read.next().unwrap().parse::<usize>().unwrap();
	let busses: Vec<Option<usize>> = read.next().unwrap().split(',').map(|s| s.parse().ok()).collect();
	let (bus, wait) = busses.iter().filter_map(|v| *v).fold((0, usize::MAX), |(prev, best), v| {
		let wait = v - (start % v);
		if wait < best {(v, wait)}
		else {(prev, best)}
	});
	let ans = bus * wait;
	println!("Bus {} in {} min. {}", bus, wait, ans);
	assert_eq!(ans, 2215);

	let busses: Vec<_> = busses.iter().enumerate().filter_map(|(idx, v)| v.map(|v| (idx, v))).collect();
	let (_, at) = busses.iter().fold((1, 0), |(step, at), (off, v)| {
		let mut at = at;
		while (at + off) % v != 0 {
			at += step;
			// println!("\t{} {}", at, (at + off) % v);
		}
		let step = lcm(step, *v);
		// println!("{} {} {}  {}", at, v, off, step);
		(step, at)
	});

	println!("{}", at);
	assert_eq!(at, 1058443396696792);
}