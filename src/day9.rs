use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::VecDeque;
use itertools::Itertools;

use std::time::{Duration, Instant};

pub fn solve() {
	let file = File::open("data/day9.txt").unwrap();
	let read = BufReader::new(file);

	let preamble = 25;
	let mut ans = 0;
	let mut buffer = VecDeque::new();
	let mut nums = Vec::new();
	for num in read.lines().map(|l| l.unwrap().parse().unwrap()) {
		nums.push(num);
		if buffer.len() >= preamble {
			if !is_any_sum_of(num, &buffer) {
				ans = num;
				println!("{}", ans);
				assert_eq!(731031916, ans);
				break;
			}
			buffer.pop_front();
		}
		buffer.push_back(num);
	}

	let mut sum = 0;
	let mut start = 0;
	let mut end = 0;
	for idx in 0..nums.len() {
		let n = nums[idx];
		end += 1;
		sum += n;
		while sum > ans {
			let v = nums[start];
			sum -= v;
			start += 1;
		}
		if sum == ans {
			let (min, max) = nums[start..(end - 2)].iter().fold((sum,0), |(min, max), n| (min.min(*n), max.max(*n)));
			let ans_b = min + max;
			println!("{}", ans_b);
			assert_eq!(93396727, ans_b);
			break;
		}
	}
}

fn is_any_sum_of(num: usize, others: &VecDeque<usize>) -> bool {
	others.iter().combinations(2).find(|v| v.iter().cloned().sum::<usize>() == num).is_some()
}