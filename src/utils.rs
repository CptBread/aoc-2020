use std::io::{BufReader, prelude::*};
use std::fs::File;

pub trait UtilRead {
	fn parse_until<T>(&mut self, until: u8) -> Option<T> 
		where T: std::str::FromStr;

	fn read_string_until(&mut self, until: u8) -> String;
}

impl<R> UtilRead for BufReader<R>
	where R: Read
{
	fn parse_until<T>(&mut self, until: u8) -> Option<T> 
		where T: std::str::FromStr
	{
		let mut buf = Vec::new();
		if self.read_until(until, &mut buf).is_ok() {
			buf.pop()?;
		}
		unsafe { std::str::from_utf8_unchecked(&buf) }.parse().ok()
	}

	fn read_string_until(&mut self, until: u8) -> String
	{
		let mut buf = Vec::new();
		if self.read_until(until, &mut buf).is_ok() {
			buf.pop();
		}
		unsafe { String::from_utf8_unchecked(buf) }
	}
}

// Maybe remove this struct and just have free loading functions...
pub struct Array2D<T> {
	pub data: Vec<T>,
	pub width: usize,
	pub height: usize,
}

impl<T> Array2D<T> {
	pub fn load_file<F>(file: File, f: F) -> Self
		where F: FnMut(char) -> T
	{
		Self::load_reader(&mut BufReader::new(file), f)
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
}