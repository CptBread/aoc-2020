use std::io::{BufReader, SeekFrom, prelude::*};
use std::fs::File;
use vek::vec::repr_c::{Vec2};

#[macro_export]
macro_rules! try_block {
	{ $($token:tt)* } => {{
		|| -> Option<()> {
			$($token)*
			Some(())
		}()
	}}
}

// Not using nightly so split once coudln't be used...
pub fn split_once<'a>(s: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
	let mut it = s.splitn(2, delim);
	Some((it.next()?, it.next()?))
}

type Pos = Vec2<usize>;

// Maybe remove this struct and just have free loading functions...
pub struct Array2D<T> {
	pub data: Vec<T>,
	pub width: usize,
	pub height: usize,
}

#[allow(dead_code)]
impl<T> Array2D<T> {
	pub fn load_read<R, F>(read: R, f: F) -> Self
		where F: FnMut(char) -> T, R: Read,
	{
		Self::load_reader(&mut BufReader::new(read), f)
	}

	pub fn load_file<F>(path: &str, f: F) -> Self
		where F: FnMut(char) -> T
	{
		let file = File::open(path).unwrap();
		Self::load_reader(&mut BufReader::new(&file), f)
	}

	pub fn load_reader<F>(read: &mut dyn BufRead, mut f: F) -> Self
		where F: FnMut(char) -> T
	{
		let mut height = 0;
		let mut width = 0;
		let mut data = Vec::new();
		for l in read.lines().map(|l| l.unwrap_or_default()) {
			let w = l.len();
			if width == 0 {
				width = w;
			}
			else if width != w {
				panic!("Inconsistent width! Assumed {} got {}", width, w);
			}
			height += 1;
			data.extend(l.chars().map(&mut f));
		}
		Array2D{
			data,
			width,
			height,
		}
	}

	pub fn to_tuple(self) -> (usize, usize, Vec<T>) {
		(self.width, self.height, self.data)
	}

	pub fn pos_to_idx(&self, pos: Pos) -> usize {
		pos.y * self.width + pos.x
	}

	pub fn wrap_pos_x(&self, pos: Pos) -> Pos {
		Vec2::new(pos.x % self.width, pos.y)
	}

	pub fn get(&self, pos: Pos) -> Option<&T> {
		self.data.get(self.pos_to_idx(pos))
	}
}