use std::collections::VecDeque;
use std::iter::FromIterator;

const INPUT: &[i8] = &[
	9,2,5,1,7,6,8,3,4
	// 3,8,9,1,2,5,4,6,7
];

pub fn solve() {
	let mut cups = VecDeque::from_iter(INPUT.iter().copied());
	for _ in 0..100 {
		let c = cups.pop_front().unwrap();
		let taken = [cups.pop_front().unwrap(), cups.pop_front().unwrap(), cups.pop_front().unwrap()];
		let dest = cups.iter().enumerate().fold((0, 99), |(bi, b), (idx, v)| {
			let d = diff(c, *v);
			if d < b { (idx, d) }
			else { (bi, b) }
		}).0;
		cups.push_back(c);
		taken.iter().rev().for_each(|v| cups.insert(dest + 1, *v));
	}
	// println!("{:?}", cups);
	let at = cups.iter().position(|v| *v == 1).unwrap();
	cups.rotate_left(at);
	cups.pop_front();
	// println!("{:?}", cups);
	for v in cups.iter() {
		print!("{}", v);
	}
	print!("\n");
	// println!("{:?}", cups);
}

fn sub(v: i8) -> i8 {
	(9+v) % 10
}

fn diff(lhs: i8, rhs: i8) -> i8 {
	(10 + lhs - rhs) % 10
}