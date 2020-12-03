// use std::fs::File;
// use std::io::{BufReader, prelude::*};
use vek::vec::repr_c::{Vec2};
use crate::utils::*;

pub fn solve()
{
	let data = Array2D::load_file("data/day3.txt", |c| c == '#');
	let count = count_trees(Vec2::new(3, 1), &data);
	println!("{}", count);
	assert_eq!(count, 151);
	let res = count 
		* count_trees(Vec2::new(1, 1), &data)
		* count_trees(Vec2::new(5, 1), &data)
		* count_trees(Vec2::new(7, 1), &data)
		* count_trees(Vec2::new(1, 2), &data)
	;
	println!("{}", res);
	assert_eq!(res, 7540141059);
}

fn count_trees(jump: Vec2<usize>, arr: &Array2D<bool>) -> usize {
	let mut at = Vec2::zero();
	let mut count = 0;
	while let Some(b) = arr.get(at) {
		count += *b as usize;
		at = arr.wrap_pos_x(at + jump);
	}
	count
}
