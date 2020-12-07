mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

use std::env;

fn main() {
	let solvers: Vec<fn()> = vec![
		day1::solve,
		day2::solve,
		day3::solve,
		day4::solve,
		day5::solve,
		day6::solve,
		day7::solve,
	];
	let day = solvers.len();
	let cmd = env::args().skip(1).next();
	let (all, day) = cmd.map_or((false, day), |s| (s == "all", s.parse::<usize>().unwrap_or(day)));
	if all {
		for (idx, f) in solvers.iter().enumerate() {
			println!("Day {}:", idx + 1);
			f();
		}
	}
	else {
		println!("Day {}:", day);
		solvers.get(day - 1).expect("Invalid day!")();
	}
}
