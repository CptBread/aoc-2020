use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::collections::{HashMap, HashSet};

// Not using nightly so split once coudln't be used...
fn split_once<'a>(s: &'a str, delim: &str) -> Option<(&'a str, &'a str)> {
	let mut it = s.splitn(2, delim);
	Some((it.next()?, it.next()?))
}

pub fn solve() {
	let file = File::open("data/day7.txt").unwrap();
	let read = BufReader::new(file);
	let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();
	let mut lookup: HashMap<String, HashSet<String>> = HashMap::new();

	for l in read.lines().map(|l| l.unwrap()) {
		let (bag, l) = split_once(&l, " bags contain ").unwrap();
		if l == "no other bags." {
			rules.insert(bag.to_string(), Vec::new());
		} else {
			let l = l.strip_suffix(".").expect("Should end with '.'");
			rules.insert(bag.to_string(), l.split(", ").map(|l| {
					let (num, bag2) = split_once(l, " ").unwrap();
					let bag2 = bag2.strip_suffix(" bags").or_else(|| bag2.strip_suffix(" bag")).expect("Should end with ' bags' or ' bag'");
					lookup.entry(bag2.to_string()).or_default().insert(bag.to_string());
					(num.parse().unwrap(), bag2.to_string())
			}).collect());
		}
	}
	let mut found = HashSet::new();
	let mut to_check = vec!["shiny gold"];
	while let Some(bag) = to_check.pop() {
		// println!("Checking {}", bag);
		if !found.insert(bag) {
			continue;
		}
		if let Some(v) = lookup.get(bag) {
			for b in v {
				if !found.contains(&b[..]) {
					to_check.push(b);
				}
			}
		}
	}
	let ans = found.len() - 1; // -1 as we also contain "shiny gold"
	println!("{}", ans);
	assert_eq!(ans, 335);

	let mut solved = HashMap::new();
	let bags = resolve_min_bags("shiny gold", &rules, &mut solved) - 1; // -1 as the function counts the shiny gold bag
	println!("{}", bags);
	assert_eq!(bags, 2431);

	// println!("{:?}", rules);
	// println!("{:?}", lookup);
}

fn resolve_min_bags<'a>(bag: &'a str, rules: &'a HashMap<String, Vec<(usize, String)>>, solved: &mut HashMap<&'a str, usize>) -> usize {
	if let Some(num) = solved.get(bag) {
		// println!("(cached){}: {}", bag, num);
		*num
	} else {
		let mut tot = 1; // include us
		for (n, b) in rules.get(bag).unwrap().iter() {
			tot += n * resolve_min_bags(b, rules, solved);
		}
		// println!("{}: {}", bag, tot);
		solved.insert(bag, tot);
		tot
	}
}