use std::fs::File;
use vek::vec::repr_c::{Vec2};
use crate::utils::*;

fn pos_to_idx(pos: Vec2<usize>, width: usize) -> usize {
	pos.y * width + pos.x
}

pub fn solve()
{
	let f = File::open("data/day3.txt").unwrap();
	let (width, _, data) = Array2D::load_file(f, |c| c == '#').to_tuple();

	let count = count_trees(Vec2::new(3, 1), &data, width);
	println!("{}", count);
	let res = count 
		* count_trees(Vec2::new(1, 1), &data, width)
		* count_trees(Vec2::new(5, 1), &data, width)
		* count_trees(Vec2::new(7, 1), &data, width)
		* count_trees(Vec2::new(1, 2), &data, width)
	;
	println!("{}", res);
}

fn count_trees(jump: Vec2<usize>, data: &Vec<bool>, width: usize) -> usize {
	let mut at = Vec2::zero();
	let mut count = 0;
	loop {
		let idx = pos_to_idx(at, width);
		if let Some(b) = data.get(idx) {
			if *b {
				count += 1;
			}
		}
		else {
			break;
		}

		at += jump;
		at.x = at.x % width;
	}
	count
}