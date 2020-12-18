use vek::vec::repr_c::{Vec2};
use crate::utils::Array2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
	Floor,
	Seat,
	Taken,
}
use Tile::*;

#[allow(dead_code)]
fn tile_to_char(t: &Tile) -> char{
	match t {
		Floor => '.',
		Seat => 'L',
		Taken => '#',
	}
}

pub fn solve() {
	let mut seats = Array2D::load_file("data/day11.txt", |c| 
		match c {
			'.' => Floor,
			'L' => Seat,
			'#' => Taken,
			_ => panic!("Unkown char!"),
		}
	);
	let org = seats.clone();
	while tick(&mut seats) != 0 {
		// seats.print(tile_to_char);
		// println!("");
	}
	let taken = seats.data.iter().fold(0, |acc, t| acc + (*t == Taken) as usize);
	println!("{}", taken);
	assert_eq!(taken, 2247);
	tick(&mut seats);
	let mut seats = org;
	while tick2(&mut seats) != 0 {
		// seats.print(tile_to_char);
		// println!("");
	}
	let taken = seats.data.iter().fold(0, |acc, t| acc + (*t == Taken) as usize);
	println!("{}", taken);
	assert_eq!(taken, 2011);
}

type Offset = Vec2<isize>;
pub const NEIGHBOUR: [Offset; 8] = [
	Offset::new(-1, -1),
	Offset::new(0, -1),
	Offset::new(1, -1),

	Offset::new(-1, 0),
	Offset::new(1, 0),

	Offset::new(-1, 1),
	Offset::new(0, 1),
	Offset::new(1, 1),
];

fn tick(seats: &mut Array2D<Tile>) -> usize {
	let mut flip = Vec::new();
	for (y, row) in seats.rows_iter().enumerate() {
		for (x, v) in row.iter().enumerate().filter(|(_, v)| **v != Floor) {
			let at = Offset::new(x as isize, y as isize);
			let near = NEIGHBOUR.iter().fold(0, |acc, off| 
				if let Some(v) = seats.get(at + off) {acc + (*v == Taken) as usize} else {acc}
			);
			if (*v == Taken && near >= 4) || (*v == Seat && near == 0) {
				flip.push(at);
			}
		}
	}
	for f in flip.iter().cloned() {
		seats.get_mut(f).map(|s| 
			*s = if *s == Taken {Seat} else {Taken}
		);
	}
	flip.len()
}

fn tick2(seats: &mut Array2D<Tile>) -> usize {
	let mut flip = Vec::new();
	for (y, row) in seats.rows_iter().enumerate() {
		for (x, v) in row.iter().enumerate().filter(|(_, v)| **v != Floor) {
			let at = Offset::new(x as isize, y as isize);
			let near = NEIGHBOUR.iter().fold(0, |acc, off| {
				let mut pos = at + off;
				while let Some(v) = seats.get(pos) {
					match *v {
						Floor => pos += *off,
						Seat => return acc,
						Taken => return acc + 1
					}
				}
				acc
			});
			if (*v == Taken && near >= 5) || (*v == Seat && near == 0) {
				flip.push(at);
			}
		}
	}
	for f in flip.iter().cloned() {
		seats.get_mut(f).map(|s| 
			*s = if *s == Taken {Seat} else {Taken}
		);
	}
	flip.len()
}