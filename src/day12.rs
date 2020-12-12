use std::fs::File;
use std::io::{BufReader, prelude::*};
// use std::collections::VecDeque;
use vek::vec::repr_c::Vec2;

fn rot(pos: Vec2<i32>, v: i32) -> Vec2<i32> {
	let v = v % 4;
	let v = if v < 0 {4 + v} else {v};
	match v {
		0 => pos,
		1 => Vec2::new(-pos.y, pos.x),
		2 => pos * -1,
		3 => Vec2::new(pos.y, -pos.x),
		_ => panic!()
	}
}

pub fn solve() {
	let file = File::open("data/day12.txt").unwrap();
	let read = BufReader::new(file);
	let lines: Vec<String> = read.lines().map(Result::unwrap).collect();
	let mut pos = Vec2::<i32>::zero();
	let mut dir = Vec2::new(1, 0);
	for l in lines.iter() {
		let c = l.chars().next().unwrap();
		let steps = l[1..].parse::<i32>().unwrap();
		let mov = match c {
			'F' => dir,
			'R' => {dir = rot(dir, steps / 90); continue},
			'L' => {dir = rot(dir, -steps / 90); continue},
			'E' => Vec2::new(1, 0),
			'W' => Vec2::new(-1, 0),
			'S' => Vec2::new(0, 1),
			'N' => Vec2::new(0, -1),
			_ => panic!("Unkown character! {}", c)
		};
		// println!("{} + {} * {} = {}", pos, mov, steps, pos + mov * steps);
		pos += mov * steps;
	}
	let ans = pos.x.abs() + pos.y.abs();
	println!("{} {}", pos, ans);
	assert_eq!(ans, 1565);

	let mut pos = Vec2::<i32>::zero();
	let mut target = Vec2::new(10, -1);
	for l in lines.iter() {
		let c = l.chars().next().unwrap();
		let steps = l[1..].parse::<i32>().unwrap();
		let mov = match c {
			'F' => {pos += target * steps; continue},
			'R' => {target = rot(target, steps / 90); continue},
			'L' => {target = rot(target, -steps / 90); continue},
			'E' => Vec2::new(1, 0),
			'W' => Vec2::new(-1, 0),
			'S' => Vec2::new(0, 1),
			'N' => Vec2::new(0, -1),
			_ => panic!("Unkown character! {}", c)
		};
		// println!("{} + {} * {} = {}", pos, mov, steps, pos + mov * steps);
		target += mov * steps;
	}
	let ans = pos.x.abs() + pos.y.abs();
	println!("{} {}", pos, ans);
	assert_eq!(ans, 78883);
}
