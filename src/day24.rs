use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use vek::vec::repr_c::Vec2;

type Pos = Vec2<i32>;
pub fn solve() {
    let file = File::open("data/day24.txt").unwrap();
    let read = BufReader::new(file);
    let lines = read.lines().map(|l| {
        l.unwrap()
            // cvrt is taken from qwerty around f
            .replace("sw", "c")
            .replace("se", "v")
            .replace("nw", "r")
            .replace("ne", "t")
    });
    let mut tiles: HashMap<Pos, bool> = HashMap::new();
    for l in lines {
        let pos = l.chars().fold(Pos::zero(), |p, c| {
            p + match c {
                'w' => Pos::new(-1, 0),
                'e' => Pos::new(1, 0),
                'r' => Pos::new(0, -1),
                't' => Pos::new(1, -1),
                'c' => Pos::new(-1, 1),
                'v' => Pos::new(0, 1),
                _ => panic!("unkown char {}", c),
            }
        });
        tiles
            .entry(pos)
            .and_modify(|b| {
                *b = !*b;
            })
            .or_insert(false);
        // println!("{:?} {}", pos, l);
    }
    let res = tiles.iter().filter(|(_, b)| !*b).count();
    println!("{:?}", res);
    assert_eq!(res, 330);
    // println!("{:?}", tiles.iter().filter(|(_, b)| **b).count());
    // println!("{:?}", tiles);
}
