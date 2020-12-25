use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use vek::vec::repr_c::{Vec2};
use crate::utils::*;

type EdgeLookup = HashMap<String, Vec<(usize, u8, bool)>>;
type Pos = Vec2<i32>;
// pub const NEIGHBOUR: [Pos; 4] = [
// 	Pos::new(-1, 0),
// 	Pos::new(0, 1),
// 	Pos::new(1, 0),
// 	Pos::new(0, -1),
// ];

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Rot(u8);
impl std::ops::Add for Rot {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self((self.0 + rhs.0) % 4)
    }
}
// impl Rot {
	
// }

pub fn solve() {
	let file = File::open("data/day20.txt").unwrap();
	let read = BufReader::new(file);
	let mut lines = read.lines().map(Result::unwrap);
	let mut tiles = HashMap::new();
	let mut edges = HashMap::new();
	let mut edge_lookup = EdgeLookup::new();
	while let Some(l) = lines.next() {
		let id = l[5..l.len() - 1].parse::<usize>().unwrap();
		let tile = Array2D::load_lines_while(&mut lines, |c| c, |l| l != "");
		let top: String = tile.data.iter().take(10).collect();
		let bot: String = tile.data.iter().skip(90).rev().collect();
		let ls: String = tile.data.iter().step_by(10).collect();
		let rs: String = tile.data.iter().skip(9).step_by(10).rev().collect();
		// println!("{} {} {} {}", top, bot, ls, rs);
		add_edges(&mut edge_lookup, &top, id, 0);
		add_edges(&mut edge_lookup, &ls, id, 1);
		add_edges(&mut edge_lookup, &bot, id, 2);
		add_edges(&mut edge_lookup, &rs, id, 3);
		edges.insert(id, [top, ls, bot, rs]);
		tiles.insert(id, tile);
	}
	// println!("{:?}", edge_lookup);
	// println!("{:?}", tiles);
	let mut corners = Vec::new();
	let mut sides = Vec::new();
	for (k, v) in edges.iter() {
		let out_edge = v.iter().fold(0, |acc, edge| 
			acc + edge_lookup[edge].iter().all(|(id, _, _)| id == k) as usize
		);
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

	let mut pic: Vec<Option<(usize, u8, bool)>> = Vec::new();
	pic.resize(tiles.len(), None);
	let mut pic = Array2D::from_vec((pic.len() as f32).sqrt() as usize, pic);
	
	// println!("{:?}", c);
	let c = corners[0];
	let mut found = !0u8;
	{
		let edges = edges.get(&c).unwrap();
		for rot in 0..4 {
			if edge_lookup[&edges[rot]].len() == 2 && edge_lookup[&edges[(rot + 1) % 4]].len() == 2 {
				found = rot as u8;
				break;
			}
		}
	}

	let mut to_place = vec![(Pos::new(0, 0), c, found)];
	while let Some((at, id, rot)) = to_place.pop() {
		*pic.get_mut(at).unwrap() = Some((id, rot, false));
		for (r, e) in edges.get(&id).unwrap().iter().enumerate() {
			if let Entry::Occupied(mut o) = edge_lookup.entry(e.clone()) {
				let v = o.get_mut();
				v.retain(|(o_id, _, _)| *o_id != id);
				if v.len() == 0 {
					o.remove();
				} else if v.len() == 1 {
					println!("CAN PLACE: {:?} from {:?}({:?})", v, (at, id, rot), r);
					// to_place.push((at + NEIGHBOUR[rot + r], ));
				}

			} else {panic!("ASDASD");}
		}
	}

	// println!("\t{:?}", found);

	// for c in corners.iter() {
	// 	println!("{:?}", c);
	// 	let mut found = !0;
	// 	let edges = edges.get(c).unwrap();
	// 	for rot in 0..4 {
	// 		if edge_lookup[&edges[rot]].len() == 2 && edge_lookup[&edges[(rot + 1) % 4]].len() == 2 {
	// 			found = rot;
	// 			break;
	// 		}
	// 	}
	// 	println!("\t{:?}", found);
	// }

	// for c in sides.iter() {
	// 	println!("{:?}", c);
	// 	for e in edges.get(c).unwrap().iter() {
	// 		println!("{:?}", edge_lookup[e]);
	// 	}
	// }

	// for e in edges.get(&corners[0]).unwrap().iter() {
	// 	println!("{:?}", edge_lookup[e]);
	// }
}

fn add_edges(lookup: &mut EdgeLookup, edge: &String, id: usize, side: u8) {
	lookup.entry(edge.clone()).or_default().push((id, side, false));
	lookup.entry(edge.chars().rev().collect()).or_default().push((id, side, true));
}

// fn rot_add(rhs: u8, lhs: u8) -> u8 {
// 	(rhs + lhs) % 4
// }

// // 0-2 1-3 2-0 3-1

// fn rot_opp(rhs: u8) -> u8 {
// 	(rhs + 2) % 4
// }