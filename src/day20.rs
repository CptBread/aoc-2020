use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;
use crate::utils::*;

type EdgeLookup = HashMap<String, Vec<(usize, u8, bool)>>;

pub fn solve() {
	let file = File::open("data/day20.txt").unwrap();
	let read = BufReader::new(file);
	let mut lines = read.lines().map(Result::unwrap);
	let mut tiles = HashMap::new();
	let mut edge_lookup = EdgeLookup::new();
	while let Some(l) = lines.next() {
		let id = l[5..l.len() - 1].parse::<usize>().unwrap();
		let tile = Array2D::load_lines_while(&mut lines, |c| c, |l| l != "");
		let top: String = tile.data.iter().take(10).collect();
		let bot: String = tile.data.iter().skip(90).collect();
		let ls: String = tile.data.iter().step_by(10).collect();
		let rs: String = tile.data.iter().skip(9).step_by(10).collect();
		// println!("{} {} {} {}", top, bot, ls, rs);
		add_edges(&mut edge_lookup, &top, id, 0);
		add_edges(&mut edge_lookup, &ls, id, 1);
		add_edges(&mut edge_lookup, &bot, id, 2);
		add_edges(&mut edge_lookup, &rs, id, 3);
		tiles.insert(id, [top, ls, bot, rs]);
	}
	// println!("{:?}", edge_lookup);
	// println!("{:?}", tiles);
	let mut corners = Vec::new();
	let mut sides = Vec::new();
	for (k, v) in tiles.iter() {
		let mut out_edge = 0;
		for edge in v {
			if edge_lookup[edge].iter().all(|(id, _, _)| id == k) {
				out_edge += 1;
			}
		}
		match out_edge {
			2 => { 
				println!("{} {}", k, out_edge);
				corners.push(*k);
			},
			1 => {
				println!("{} {}", k, out_edge);
				sides.push(*k);
			}
			_ => {},
		}
	}
	assert_eq!(corners.len(), 4);
	let res = corners.iter().product::<usize>();
	println!("{}", res);
	assert_eq!(res, 174206308298779);
}

fn add_edges(lookup: &mut EdgeLookup, edge: &String, id: usize, side: u8) {
	lookup.entry(edge.clone()).or_default().push((id, side, false));
	lookup.entry(edge.chars().rev().collect()).or_default().push((id, side, true));
}