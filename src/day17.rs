use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;
use vek::vec::repr_c::{Vec3, Vec4};

#[derive(Debug, Default, Clone)]
struct Tile {
	active: bool,
	count: u8,
}

// struct Tiles {
// 	data: VecDeque<Array2D<Tile>>,
// 	start: Vec3<isize>,
// }

// type Pos = Vec3<i32>;

pub const NEIGHBOUR: &[Vec3<i32>] = &[
	// Top
	Vec3::new(-1, -1, 1),
	Vec3::new(0, -1, 1),
	Vec3::new(1, -1, 1),

	Vec3::new(-1, 0, 1),
	Vec3::new(0, 0, 1),
	Vec3::new(1, 0, 1),

	Vec3::new(-1, 1, 1),
	Vec3::new(0, 1, 1),
	Vec3::new(1, 1, 1),

	// Mid
	Vec3::new(-1, -1, 0),
	Vec3::new(0, -1, 0),
	Vec3::new(1, -1, 0),

	Vec3::new(-1, 0, 0),
	Vec3::new(1, 0, 0),

	Vec3::new(-1, 1, 0),
	Vec3::new(0, 1, 0),
	Vec3::new(1, 1, 0),

	// Bottom
	Vec3::new(-1, -1, -1),
	Vec3::new(0, -1, -1),
	Vec3::new(1, -1, -1),

	Vec3::new(-1, 0, -1),
	Vec3::new(0, 0, -1),
	Vec3::new(1, 0, -1),

	Vec3::new(-1, 1, -1),
	Vec3::new(0, 1, -1),
	Vec3::new(1, 1, -1),
];

pub const NEIGHBOUR4: &[Vec4<i32>] = &[
	// MID
	// Top
	Vec4::new(-1, -1, 1, 0),
	Vec4::new(0, -1, 1, 0),
	Vec4::new(1, -1, 1, 0),

	Vec4::new(-1, 0, 1, 0),
	Vec4::new(0, 0, 1, 0),
	Vec4::new(1, 0, 1, 0),

	Vec4::new(-1, 1, 1, 0),
	Vec4::new(0, 1, 1, 0),
	Vec4::new(1, 1, 1, 0),

	// Mid
	Vec4::new(-1, -1, 0, 0),
	Vec4::new(0, -1, 0, 0),
	Vec4::new(1, -1, 0, 0),

	Vec4::new(-1, 0, 0, 0),
	Vec4::new(1, 0, 0, 0),

	Vec4::new(-1, 1, 0, 0),
	Vec4::new(0, 1, 0, 0),
	Vec4::new(1, 1, 0, 0),

	// Bottom
	Vec4::new(-1, -1, -1, 0),
	Vec4::new(0, -1, -1, 0),
	Vec4::new(1, -1, -1, 0),

	Vec4::new(-1, 0, -1, 0),
	Vec4::new(0, 0, -1, 0),
	Vec4::new(1, 0, -1, 0),

	Vec4::new(-1, 1, -1, 0),
	Vec4::new(0, 1, -1, 0),
	Vec4::new(1, 1, -1, 0),


	// TOP
	// Top
	Vec4::new(-1, -1, 1, 1),
	Vec4::new(0, -1, 1, 1),
	Vec4::new(1, -1, 1, 1),

	Vec4::new(-1, 0, 1, 1),
	Vec4::new(0, 0, 1, 1),
	Vec4::new(1, 0, 1, 1),

	Vec4::new(-1, 1, 1, 1),
	Vec4::new(0, 1, 1, 1),
	Vec4::new(1, 1, 1, 1),

	// Mid
	Vec4::new(-1, -1, 0, 1),
	Vec4::new(0, -1, 0, 1),
	Vec4::new(1, -1, 0, 1),

	Vec4::new(-1, 0, 0, 1),
	Vec4::new(0, 0, 0, 1),
	Vec4::new(1, 0, 0, 1),

	Vec4::new(-1, 1, 0, 1),
	Vec4::new(0, 1, 0, 1),
	Vec4::new(1, 1, 0, 1),

	// Bottom
	Vec4::new(-1, -1, -1, 1),
	Vec4::new(0, -1, -1, 1),
	Vec4::new(1, -1, -1, 1),

	Vec4::new(-1, 0, -1, 1),
	Vec4::new(0, 0, -1, 1),
	Vec4::new(1, 0, -1, 1),

	Vec4::new(-1, 1, -1, 1),
	Vec4::new(0, 1, -1, 1),
	Vec4::new(1, 1, -1, 1),

	// BOTTOM
	// Top
	Vec4::new(-1, -1, 1, -1),
	Vec4::new(0, -1, 1, -1),
	Vec4::new(1, -1, 1, -1),

	Vec4::new(-1, 0, 1, -1),
	Vec4::new(0, 0, 1, -1),
	Vec4::new(1, 0, 1, -1),

	Vec4::new(-1, 1, 1, -1),
	Vec4::new(0, 1, 1, -1),
	Vec4::new(1, 1, 1, -1),

	// Mid
	Vec4::new(-1, -1, 0, -1),
	Vec4::new(0, -1, 0, -1),
	Vec4::new(1, -1, 0, -1),

	Vec4::new(-1, 0, 0, -1),
	Vec4::new(0, 0, 0, -1),
	Vec4::new(1, 0, 0, -1),

	Vec4::new(-1, 1, 0, -1),
	Vec4::new(0, 1, 0, -1),
	Vec4::new(1, 1, 0, -1),

	// Bottom
	Vec4::new(-1, -1, -1, -1),
	Vec4::new(0, -1, -1, -1),
	Vec4::new(1, -1, -1, -1),

	Vec4::new(-1, 0, -1, -1),
	Vec4::new(0, 0, -1, -1),
	Vec4::new(1, 0, -1, -1),

	Vec4::new(-1, 1, -1, -1),
	Vec4::new(0, 1, -1, -1),
	Vec4::new(1, 1, -1, -1),
];

#[allow(dead_code)]
fn print_vol(start: Vec3<i32>, end: Vec3<i32>, data: &HashMap<Vec3<i32>, Tile>) {
	for z in start.z..=end.z {
		println!("Slice: {}", z);
		for y in start.y..=end.y {
			for x in start.x..=end.x {
				if data.get(&Vec3::new(x, y, z)).map_or(false, |t| t.active) {
					print!("#");
				} else {
					print!(".");
				}
			}
			print!("\n");
		}
	}
}

#[allow(dead_code)]
fn print_hyper(start: Vec4<i32>, end: Vec4<i32>, data: &HashMap<Vec4<i32>, Tile>) {
	for w in start.w..=end.w {
		for z in start.z..=end.z {
			println!("Slice: z: {} w: {}", z, w);
			for y in start.y..=end.y {
				for x in start.x..=end.x {
					if data.get(&Vec4::new(x, y, z, w)).map_or(false, |t| t.active) {
						print!("#");
					} else {
						print!(".");
					}
				}
				print!("\n");
			}
		}
	}
}

pub fn solve() {
	part1();
	part2();	
}

fn part1() {
	let file = File::open("data/day17.txt").unwrap();
	let read = BufReader::new(file);
	let mut tiles: HashMap<Vec3<i32>, Tile> = HashMap::new();
	// let mut tiles = HashMap::new();
	let mut min = Vec3::zero();
	let mut max = Vec3::zero();
	read.lines().enumerate().for_each(|(y, l)| {
		let l = l.unwrap();
		l.chars().enumerate().for_each(|(x, c)| {
			if c == '#' {
				let at = Vec3::new(x as i32, y as i32, 0);
				min = Vec3::min(min,at);
				max = Vec3::max(max, at);
				tiles.entry(at).or_default().active = true;
				for n in NEIGHBOUR.iter() {
					tiles.entry(at + n).or_default().count += 1;
				}
			}
		});
	});
	// print_vol(min, max, &tiles);

	let mut next: HashMap<_, Tile> = HashMap::new();
	let mut count = 0;
	for _ in 0..6 {
		count = 0;
		min = Vec3::zero();
		max = Vec3::zero();
		for (k, v) in tiles.iter() {
			if (v.active && (2..=3).contains(&v.count)) || (!v.active && v.count == 3) {
				count += 1;
				let at = *k;
				min = Vec3::min(min, at);
				max = Vec3::max(max, at);
				next.entry(at).or_default().active = true;
				for n in NEIGHBOUR.iter() {
					next.entry(at + n).or_default().count += 1;
				}
			}
		}
		std::mem::swap(&mut tiles, &mut next);
		next.clear();
	}

	// print_vol(min, max, &tiles);
	println!("{}", count);
	assert_eq!(count, 207);
	// println!("{:?}", tiles);
}

fn part2() {
	let file = File::open("data/day17.txt").unwrap();
	let read = BufReader::new(file);
	let mut tiles: HashMap<Vec4<i32>, Tile> = HashMap::new();
	// let mut tiles = HashMap::new();
	let mut min = Vec4::zero();
	let mut max = Vec4::zero();
	read.lines().enumerate().for_each(|(y, l)| {
		let l = l.unwrap();
		l.chars().enumerate().for_each(|(x, c)| {
			if c == '#' {
				let at = Vec4::new(x as i32, y as i32, 0, 0);
				min = Vec4::min(min,at);
				max = Vec4::max(max, at);
				tiles.entry(at).or_default().active = true;
				for n in NEIGHBOUR4.iter() {
					tiles.entry(at + n).or_default().count += 1;
				}
			}
		});
	});

	let mut next: HashMap<_, Tile> = HashMap::new();
	let mut count = 0;
	for _ in 0..6 {
		count = 0;
		min = Vec4::zero();
		max = Vec4::zero();
		for (k, v) in tiles.iter() {
			if (v.active && (2..=3).contains(&v.count)) || (!v.active && v.count == 3) {
				count += 1;
				let at = *k;
				min = Vec4::min(min,at);
				max = Vec4::max(max, at);
				next.entry(at).or_default().active = true;
				for n in NEIGHBOUR4.iter() {
					next.entry(at + n).or_default().count += 1;
				}
			}
		}
		std::mem::swap(&mut tiles, &mut next);
		next.clear();
	}

	// print_hyper(min, max, &tiles);
	println!("{}", count);
	assert_eq!(count, 2308);
}