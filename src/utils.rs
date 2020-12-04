use std::io::{BufReader, SeekFrom, prelude::*};
use std::fs::File;
use vek::vec::repr_c::{Vec2};

pub trait UtilRead {
	fn parse_until<T>(&mut self, until: u8) -> Option<T> 
		where T: std::str::FromStr
	{
		let mut buf = Vec::new();
		self.parse_until_buf(until, &mut buf)
	}
	fn parse_until_buf<T>(&mut self, until: u8, buf: &mut Vec<u8>) -> Option<T> 
		where T: std::str::FromStr
	{
		self.read_to_buf_until(until, buf)?;
		unsafe { std::str::from_utf8_unchecked(&buf) }.parse().ok()
	}

	fn read_string_until(&mut self, until: u8) -> String
	{
		let mut buf = Vec::new();
		self.read_to_buf_until(until, &mut buf);
		unsafe { String::from_utf8_unchecked(buf) }
	}

	fn read_to_buf_until(&mut self, until: u8, buf: &mut Vec<u8>) -> Option<usize>;
	fn read_byte(&mut self) -> Option<u8>;
}

pub trait UtilReadSeek {
	fn parse_while<T, F>(&mut self, f: F) -> Option<T>
		where T: std::str::FromStr, F: FnMut(u8) -> bool
	{
		let mut buf = Vec::new();
		self.parse_while_buf(&mut buf, f)
	}

	fn parse_while_buf<T, F>(&mut self, buf: &mut Vec<u8>, f: F) -> Option<T> 
		where T: std::str::FromStr, F: FnMut(u8) -> bool
	{
		self.read_to_buf_while(buf, f)?;
		unsafe{ std::str::from_utf8_unchecked(&buf) }.parse().ok()
	}

	fn parse_uint<T>(&mut self) -> Option<T> 
	where T: std::str::FromStr
	{
		let mut buf = Vec::new();
		self.parse_uint_buf(&mut buf)
	}

	fn parse_int<T>(&mut self) -> Option<T> 
		where T: std::str::FromStr
	{
		let mut buf = Vec::new();
		self.parse_int_buf(&mut buf)
	}

	fn parse_float<T>(&mut self) -> Option<T> 
		where T: std::str::FromStr
	{
		let mut buf = Vec::new();
		self.parse_float_buf(&mut buf)
	}

	fn parse_uint_buf<T>(&mut self, buf: &mut Vec<u8>) -> Option<T> 
		where T: std::str::FromStr
	{
		self.parse_while_buf(buf, |b| b.is_ascii_digit())
	}

	fn parse_int_buf<T>(&mut self, buf: &mut Vec<u8>) -> Option<T> 
		where T: std::str::FromStr
	{
		let mut first = true;
		self.parse_while_buf(buf, |b| if first {first = false; b == b'-'} else {false} || b.is_ascii_digit())
	}

	fn parse_float_buf<T>(&mut self, buf: &mut Vec<u8>) -> Option<T> 
		where T: std::str::FromStr
	{
		let mut first = true;
		let mut no_decimal = true;
		self.parse_while_buf(buf, |b| 
			if first {first = false; b == b'-'} else {false} || // first can be - to deal with negatives
			if no_decimal && b == b'.' {no_decimal = false; true} else {false} || // only one . is allowed
			b.is_ascii_digit()
		)
	}

	fn read_to_buf_while<F>(&mut self, buf: &mut Vec<u8>, f: F) -> Option<usize>
		where F: FnMut(u8) -> bool;
}

impl<R> UtilRead for BufReader<R>
	where R: Read
{
	fn read_to_buf_until(&mut self, until: u8, buf: &mut Vec<u8>) -> Option<usize> {
		let not = until.wrapping_add(1);
		if let Ok(read) = self.read_until(until, buf) {
			if *buf.last().unwrap_or(&not) == until {buf.pop();};
			Some(read)
		} else{
			None
		}
	}

	fn read_byte(&mut self) -> Option<u8>
	{
		let mut byte = [0];
		if let Ok(r) = self.read(&mut byte) {
			Some(byte[0])
		} else {
			None
		}
	}

}

impl<R> UtilReadSeek for BufReader<R>
	where R: Read+Seek
{
	fn read_to_buf_while<F>(&mut self, buf: &mut Vec<u8>, mut f: F) -> Option<usize>
		where F: FnMut(u8) -> bool
	{
		let mut read = 0;
		while let Some(b) = self.read_byte() {
			if f(b) {
				read += 1;
				buf.push(b);
			} else {
				self.seek(SeekFrom::Current(-1)).ok()?;
				return Some(read);
			}
		}
		Some(read)
	}
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

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Cursor;

	#[test]
	fn test_parse_int() {
		assert_eq!(BufReader::new(Cursor::new(b"9")).parse_int::<i32>(), Some(9));
		assert_eq!(BufReader::new(Cursor::new(b"0132")).parse_int::<i32>(), Some(132));
		assert_eq!(BufReader::new(Cursor::new(b"-2")).parse_int::<i32>(), Some(-2));
		assert_eq!(BufReader::new(Cursor::new(b"0.1")).parse_int::<i32>(), Some(0));
		assert_eq!(BufReader::new(Cursor::new(b"--2")).parse_int::<i32>(), None);
		assert_eq!(BufReader::new(Cursor::new(b"-")).parse_int::<i32>(), None);
		// Fails on too large numbers
		assert_eq!(BufReader::new(Cursor::new(b"999999999999999999")).parse_int::<i32>(), None);
	}

	#[test]
	fn test_parse_uint() {
		assert_eq!(BufReader::new(Cursor::new(b"9")).parse_uint::<u32>(), Some(9));
		assert_eq!(BufReader::new(Cursor::new(b"0132")).parse_uint::<u32>(), Some(132));
		assert_eq!(BufReader::new(Cursor::new(b"-2")).parse_uint::<u32>(), None);
		assert_eq!(BufReader::new(Cursor::new(b"-2")).parse_uint::<i32>(), None);
		assert_eq!(BufReader::new(Cursor::new(b"0.1")).parse_uint::<u32>(), Some(0));
		assert_eq!(BufReader::new(Cursor::new(b"--2")).parse_uint::<u32>(), None);
		assert_eq!(BufReader::new(Cursor::new(b"-")).parse_uint::<u32>(), None);
		// Fails on too large numbers
		assert_eq!(BufReader::new(Cursor::new(b"999999999999999999")).parse_uint::<u32>(), None);
	}

	#[test]
	fn test_parse_float() {
		assert_eq!(BufReader::new(Cursor::new(b"9")).parse_float::<f32>(), Some(9.0));
		assert_eq!(BufReader::new(Cursor::new(b"0132")).parse_float::<f32>(), Some(132.0));
		assert_eq!(BufReader::new(Cursor::new(b"1.32")).parse_float::<f32>(), Some(1.32));
		assert_eq!(BufReader::new(Cursor::new(b"-2")).parse_float::<f32>(), Some(-2.0));
		assert_eq!(BufReader::new(Cursor::new(b"0.1")).parse_float::<f32>(), Some(0.1));
		assert_eq!(BufReader::new(Cursor::new(b"0..1")).parse_float::<f32>(), Some(0.0));
		assert_eq!(BufReader::new(Cursor::new(b"0..1")).parse_float::<f32>(), Some(0.0));
		assert_eq!(BufReader::new(Cursor::new(b"-0.1")).parse_float::<f32>(), Some(-0.1));
		assert_eq!(BufReader::new(Cursor::new(b"--2")).parse_float::<f32>(), None);
		assert_eq!(BufReader::new(Cursor::new(b"-")).parse_float::<f32>(), None);
		assert_eq!(BufReader::new(Cursor::new(b".")).parse_float::<f32>(), None);
	}
}