use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::{HashMap, HashSet, BTreeMap};
use crate::utils::*;

pub fn solve() {
	let file = File::open("data/day21.txt").unwrap();
	let read = BufReader::new(file);
	let lines = read.lines().map(Result::unwrap);
	// let mut parts: HashMap::new();
	let mut parts: HashSet<String> = HashSet::new();
	let mut times: HashMap<String, usize> = HashMap::new();
	let mut allergen: HashMap<String, HashSet<String>> = HashMap::new();
	for l in lines {
		let (l, all) = split_once(&l, " (").unwrap();
		let all = &all["contains ".len()..all.len()-1];
		let all: Vec<_> = all.split(", ").collect();
		let ing: HashSet<_> = l.split(' ').map(|s| s.to_string()).inspect(|s| *times.entry(s.clone()).or_default() += 1).collect();
		parts = parts.union(&ing).cloned().collect();
		for a in all {
			allergen.entry(a.to_string()).and_modify(|a| *a = a.intersection(&ing).cloned().collect()).or_insert(ing.clone());
		}
	}
	// println!("{:?}", allergen);
	let mut no_all = parts.clone();
	for v in allergen.values() {
		for v in v.iter() {
			no_all.remove(v);
		}
	}
	// println!("{:?}", no_all);
	let mut res = 0;
	for i in no_all.iter() {
		res += times[i];
	}
	println!("{:?}", res);
	assert_eq!(res, 2078);
	let mut known = BTreeMap::new();
	while let Some((k, v)) = allergen.iter().find_map(|(k, v)| if v.len() == 1 {Some((k.clone(), v.iter().next()?.clone()))} else {None}) {
		allergen.remove(&k);
		allergen.iter_mut().for_each(|(_, l)| {l.remove(&v);});
		known.insert(k, v);
	}
	// println!("{:?}", known);
	let mut res = "".to_string();
	for (_, v) in known.iter() {
		res += &format!("{},", &v[..]);
	}
	res.pop();
	println!("{}", res);
	assert_eq!(&res, "lmcqt,kcddk,npxrdnd,cfb,ldkt,fqpt,jtfmtpd,tsch");
}