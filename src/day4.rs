use std::fs::File;
use std::io::prelude::*;

const FIELDS: [&str;8] = [
	"byr",
	"iyr",
	"eyr",
	"hgt",
	"hcl",
	"ecl",
	"pid",
	"cid",
];

const COLORS: [&str; 7] = [
	"amb",
	"blu",
	"brn",
	"gry",
	"grn",
	"hzl",
	"oth",
];

pub fn solve() {
	let mut file = File::open("data/day4.txt").unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content).expect("Failed to read file!");

	let mut count = 0;
	let mut count_b = 0;
	let mut found = 0;
	let mut checks = 0;
	// let mut buf = Vec::new();
	for item in content.split(&[' ', '\n'][..]) {
		// println!("{}", item);
		if item == "\r" {
			count += ((found & 0b1111111) == 0b1111111) as usize;
			count_b += ((checks & 0b1111111) == 0b1111111) as usize;
			// println!("{:b} {:b} {} {}", found, checks, ok, check_ok);
			// println!("{:b} {:b}", found, checks);
			found = 0;
			checks = 0;
		} else {
			let mut it = item.trim_end_matches('\r').split(':');
			let cat = it.next().unwrap();
			let data = it.next().unwrap_or("");
			if let Some(idx) = FIELDS.iter().position(|s| *s == cat) {
				found |= 1 << idx;
				match idx {
					0 => {
						assert_eq!(FIELDS[idx], "byr");
						let year = data.parse().unwrap_or(0);
						let ok = year >= 1920 && year <= 2002;
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					}
					1 => {
						assert_eq!(FIELDS[idx], "iyr");
						let year = data.parse().unwrap_or(0);
						let ok = year >= 2010 && year <= 2020;
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					}
					2 => {
						assert_eq!(FIELDS[idx], "eyr");
						let year = data.parse().unwrap_or(0);
						let ok = year >= 2020 && year <= 2030;
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					}
					3 => {
						assert_eq!(FIELDS[idx], "hgt");
						let ok = if let Some(h) = data.strip_suffix("in").and_then(|s| s.parse::<usize>().ok()) { h >= 59 && h <= 76 }
							else if let Some(h) = data.strip_suffix("cm").and_then(|s| s.parse::<usize>().ok()) { h >= 150 && h <= 193 }
							else { false }
						;
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					}
					4 => {
						assert_eq!(FIELDS[idx], "hcl");
						let ok = data.len() == 7 && &data[0..1] == "#" &&
							data[1..].bytes().all(|b| b.is_ascii_digit() || b.is_ascii_lowercase())
						;
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					}
					5 => {
						assert_eq!(FIELDS[idx], "ecl");
						let ok = COLORS.iter().position(|f| f == &data).is_some();
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					}
					6 => {
						assert_eq!(FIELDS[idx], "pid");
						let ok = data.len() == 9 && data.chars().all(|b| b.is_ascii_digit());
						checks |= (ok as usize) << idx;
						// println!("\t{} {}:{}", ok, cat, data);
					} 
					_ => {
						// r.read_until(b' ', &mut buf).unwrap();
					}
				}
			}

		}
	}
	count += ((found & 0b1111111) == 0b1111111) as usize;
	count_b += ((checks & 0b1111111) == 0b1111111) as usize;
	// println!("{:b} {:b}", found, checks);

	println!("{} {}", count, count_b);
	assert_eq!(count, 222);
	assert_eq!(count_b, 140);
}