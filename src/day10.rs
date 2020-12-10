use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;

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

	let mut cache = HashMap::new();
	cache.insert(adapters.len() - 1, 1);
	let options = check_opts(0, &mut cache, &adapters);
	println!("{}", options);
	assert_eq!(options, 3022415986688);
}

fn check_opts(at: usize, cache: &mut HashMap<usize, usize>, adapters: &Vec<i32>) -> usize {
	if let Some(v) = cache.get(&at) {
		return *v;
	}
	let mut res = 0;
	let num = adapters[at];
	for idx in at + 1..(at + 4).min(adapters.len()) {
		let v = adapters[idx];
		if v - num <= 3 {
			res += check_opts(idx, cache, adapters);
		} else {
			break;
		}
	}
	cache.insert(at, res);
	res
}