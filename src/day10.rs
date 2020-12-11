use std::fs::File;
use std::io::{BufReader, prelude::*};

// The amount of ways to traverse between the "one islands" of diffrent sizes follows the tribonacci sequence
const TRIBONACCI: &[usize] = &[
	1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274, 504, 927, 1705, 3136, 5768, 10609, 19513, 35890, 66012, 121415, 223317, 410744, 755476, 1389537, 2555757, 4700770, 8646064, 15902591, 29249425, 53798080, 98950096, 181997601, 334745777, 615693474, 1132436852,
];

pub fn solve() {
	let file = File::open("data/day10.txt").unwrap();
	let mut adapters: Vec<_> = BufReader::new(file).lines()
		.map(|l| l.unwrap().parse::<i32>().unwrap()).collect()
	;
	adapters.sort();
	adapters.insert(0, 0);
	adapters.push(adapters.last().unwrap() + 3);

	let mut diffs = [0, 0, 0];
	for win in adapters.windows(2) {
		let (v0, v1) = (win[0], win[1]);
		let idx = (v1 - v0) - 1;
		diffs[idx as usize] += 1;
	}
	let ans = diffs[0] * diffs[2];
	println!("{}", ans);
	assert_eq!(ans, 2112);

	// FYI, this is not how I first solved this. First I didn't rely on the only 1 or 3 diff thing
	// so I brute force calculated each posibility but also used a cache so to not recalculate the same
	// parts over and over again.
	let ans = adapters.windows(2).fold((1usize, 0usize), |(res, size), v| 
		if v[0] + 1 == v[1] {(res, size + 1)}
		else {(res * TRIBONACCI[size], 0)}
	).0;
	println!("{}", ans);
	assert_eq!(ans, 3022415986688);
}