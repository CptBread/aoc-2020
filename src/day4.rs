use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::iter;
use crate::utils::*;

const FIELDS: [&[u8];8] = [
	b"byr",
	b"iyr",
	b"eyr",
	b"hgt",
	b"hcl",
	b"ecl",
	b"pid",
	b"cid",
];

const COLORS: [&[u8]; 7] = [
	b"amb",
	b"blu",
	b"brn",
	b"gry",
	b"grn",
	b"hzl",
	b"oth",
];

pub fn solve() {
	let file = File::open("data/day4.txt").unwrap();
	let mut reader = BufReader::new(file);

	let mut count = 0;
	let mut count_b = 0;
	let mut maybe_end = false;
	let mut found = 0;
	let mut checks = 0;
	let mut buf = Vec::new();
	for l in reader.lines().map(|l| l.unwrap()).chain(iter::once(String::from(""))) {
		let mut r = BufReader::new(l.as_bytes());
		loop {
			buf.clear();
			let read = r.read_until(b':', &mut buf).unwrap_or(0);
			if read == 0 {
				if maybe_end {
					// let mut ok = false;
					// let mut check_ok = false;
					if (found & 0b1111111) == 0b1111111 {
						count += 1;
						// ok = true;
						if (checks & 0b1111111) == 0b1111111 {
							count_b += 1;
							// check_ok = true;
						}
					}
					// println!("{:b} {:b} {} {}", found, checks, ok, check_ok);
					maybe_end = false;
					found = 0;
					checks = 0;
				} else {
					maybe_end = true;
				}
				break;
			} else {
				maybe_end = false;
				buf.resize(3, 0);
				if let Some(idx) = FIELDS.iter().position(|f| *f == buf) {
					found |= 1 << idx;
					let cat = String::from_utf8_lossy(&buf).into_owned();
					if buf == b"hgt" {
						buf.clear();
						let read = r.read_until(b' ', &mut buf).unwrap();
						if read > 3 {
							if *buf.last().unwrap() == b' ' { buf.pop(); };
							let (c1, c0) = (buf.pop().unwrap(), buf.pop().unwrap());
							let end = [c0, c1];
							let h = unsafe {std::str::from_utf8_unchecked(&buf)}.parse::<usize>().unwrap_or(0);
							let ok = (&end == b"in" && h >= 59 && h <= 76)
								|| (&end == b"cm" && h >= 150 && h <= 193)
							;
							checks |= (ok as usize) << idx;
							// println!("\t{} {:?}", ok, cat);
						}
						else {
							let ok = false;
							// println!("\t{} {:?}", ok, cat);
						}
					}
					else if buf == b"byr" {
						let year = r.parse_until(b' ').unwrap_or(0);
						let ok = year >= 1920 && year <= 2002;
						checks |= (ok as usize) << idx;
						// println!("\t{} {:?}", ok, cat);
					}
					else if buf == b"iyr" {
						let year = r.parse_until(b' ').unwrap_or(0);
						let ok = year >= 2010 && year <= 2020;
						checks |= (ok as usize) << idx;
						// println!("\t{} {:?}", ok, cat);
					}
					else if buf == b"eyr" {
						let year = r.parse_until(b' ').unwrap_or(0);
						let ok = year >= 2020 && year <= 2030;
						checks |= (ok as usize) << idx;
						// println!("\t{} {:?}", ok, cat);
					}
					else if buf == b"hcl" {
						buf.clear();
						let read = r.read_until(b' ', &mut buf).unwrap_or(0);
						if *buf.last().unwrap() == b' ' { buf.pop(); };
						let ok = buf.len() == 7 && buf.remove(0) == b'#' && buf.iter().all(|b| (*b >= b'0' && *b <= b'9') || (*b >= b'a' && *b <= b'z'));
						checks |= (ok as usize) << idx;
						// println!("\t{} {:?}", ok, cat);
					}
					else if buf == b"ecl" {
						buf.clear();
						r.read_until(b' ', &mut buf).unwrap_or(0);
						if *buf.last().unwrap() == b' ' { buf.pop(); };
						let ok = COLORS.iter().position(|f| *f == buf).is_some();
						checks |= (ok as usize) << idx;
						// println!("\t{} {:?}", ok, cat);
					}
					else if buf == b"pid" {
						buf.clear();
						let read = r.read_until(b' ', &mut buf).unwrap_or(0);
						if *buf.last().unwrap() == b' ' { buf.pop(); };
						let ok = buf.len() == 9 && buf.iter().all(|b| (*b >= b'0' && *b <= b'9'));
						checks |= (ok as usize) << idx;
						// println!("\t{} {:?}", ok, cat);
					} else {
						r.read_until(b' ', &mut buf).unwrap();
						// println!("\tignore {:?}", cat);
					}
				}
				else {
					println!("Bad category: {:?}", std::str::from_utf8(&buf[..]));
				}
			}
		}
	}
	println!("{} {}", count, count_b);
	assert_eq!(count, 222);
	assert_eq!(count_b, 140);
}