use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::{VecDeque, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn solve() {
	let file = File::open("data/day22.txt").unwrap();
	let read = BufReader::new(file);
	let mut lines = read.lines().map(Result::unwrap);
	let mut deck0: VecDeque<_> = lines.by_ref().skip(1).take_while(|l| l != "").map(|l| l.parse::<usize>().unwrap()).collect();
	let mut deck1: VecDeque<_> = lines.by_ref().skip(1).take_while(|l| l != "").map(|l| l.parse::<usize>().unwrap()).collect();
	// println!("{:?}", deck0);
	// println!("{:?}", deck1);
	let d0 = deck0.clone();
	let d1 = deck1.clone();

	while deck0.len() != 0 && deck1.len() != 0 {
		let c0 = deck0.pop_front().unwrap();
		let c1 = deck1.pop_front().unwrap();
		if c0 > c1 {
			deck0.push_back(c0);
			deck0.push_back(c1);
		} else {
			deck1.push_back(c1);
			deck1.push_back(c0);
		}
	}
	let win = if deck0.len() > deck1.len() {&deck0} else {&deck1};
	let res = win.iter().fold((0, win.len()), |(acc, i), v| {
		(acc + v * i, i - 1)
	}).0;
	println!("{:?}", res);
	assert_eq!(res, 33925);

	let mut deck0 = d0;
	let mut deck1 = d1;
	let win = play(&mut deck0, &mut deck1);
	let win = if win {&deck0} else {&deck1};
	let res = win.iter().fold((0, win.len()), |(acc, i), v| {
		(acc + v * i, i - 1)
	}).0;
	println!("{:?}", res);
	assert_eq!(res, 33441);
}

fn play(deck0:&mut VecDeque<usize>, deck1:&mut VecDeque<usize>) -> bool {
	let mut prev_decks = HashSet::new();
	loop {
		if deck0.len() == 0 {
			return false;
		} else if deck1.len() == 0 {
			return true;
		}
		if !prev_decks.insert(hash(deck0, deck1)) {
			return true;
		}
		let c0 = deck0.pop_front().unwrap();
		let c1 = deck1.pop_front().unwrap();
		let mut win = c0 > c1;
		if c0 <= deck0.len() && c1 <= deck1.len() {
			let mut d0 = deck0.iter().take(c0).cloned().collect();
			let mut d1 = deck1.iter().take(c1).cloned().collect();
			win = play(&mut d0, &mut d1);
		}
		if win {
			deck0.push_back(c0);
			deck0.push_back(c1);
		} else {
			deck1.push_back(c1);
			deck1.push_back(c0);
		}
	}
}

fn hash(d0: &VecDeque<usize>, d1: &VecDeque<usize>) -> u64 {
	let mut hasher = DefaultHasher::new();
	d0.hash(&mut hasher);
	d1.hash(&mut hasher);
	hasher.finish()
}