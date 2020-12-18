use std::fs::File;
use std::io::{BufReader, prelude::*};

pub fn solve() {
	let file = File::open("data/day18.txt").unwrap();
	let read = BufReader::new(file);
	let mut sum = 0;
	let mut sum2 = 0;
	for l in read.lines().map(Result::unwrap) {
		let mut it = l.chars().filter(|c| *c != ' ');
		let mut it2 = it.clone();
		let res = eval(&mut it);
		// println!("{:?}", res);
		sum += res.unwrap();
		let res = eval2(&mut it2);
		// println!("{:?}", res);
		sum2 += res.unwrap();
	}
	println!("{}", sum);
	assert_eq!(sum, 30753705453324);
	println!("{}", sum2);
	assert_eq!(sum2, 244817530095503);
}

fn eval<I>(it: &mut I) -> Option<u64> 
	where I: Iterator<Item = char>
{
	let mut curr = get_val(it, eval)?;
	loop {
		curr = match it.next() {
			Some(')') => return Some(curr),
			Some('+') => curr + get_val(it, eval).expect("No rhs value for '+'"),
			Some('*') => curr * get_val(it, eval).expect("No rhs value for '*'"),
			Some(_) => panic!("Expected an operator!"),
			None => return Some(curr),
		}
	}
}

fn get_val<I, F>(it: &mut I, func: F) -> Option<u64>
	where 
		I: Iterator<Item = char>,
		F: Fn(&mut I) -> Option<u64>
{
	match it.next()? {
		'(' => func(it),
		c => c.to_digit(10).map(|v| v as u64),
	}
}

fn eval2<I>(it: &mut I) -> Option<u64> 
	where I: Iterator<Item = char>
{
	let mut curr = get_val(it, eval2)?;
	loop {
		curr = match it.next() {
			Some('+') => curr + get_val(it, eval2).expect("No rhs value for '+'"),
			Some('*') => return Some(curr * eval2(it).expect("No rhs value for '*'")),
			None | Some(')') => return Some(curr),
			Some(_) => panic!("Expected an operator!"),
		}
	}
}