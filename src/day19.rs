use std::fs::File;
use std::io::{BufReader, prelude::*};
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Match {
	Char(char),
	Rule(usize),
}

#[derive(Clone, Debug)]
struct Rule {
	matches: Vec<Match>,
	or: Option<usize>,
}

pub fn solve() {
	let file = File::open("data/day19.txt").unwrap();
	let read = BufReader::new(file);
	let mut lines = read.lines().map(Result::unwrap);
	let mut rules = Vec::new();
	while let Some(l) = lines.next() {
		if l == "" {
			break;
		}
		let (id, l) = split_once(&l, ": ").and_then(|(id, l)| Some((id.parse::<usize>().ok()?, l))).unwrap();
		if id >= rules.len() {
			rules.resize(id + 1, Rule{matches: Vec::new(), or: None});
		}
		let rule = &mut rules[id];
		for s in l.split(' ') {
			match s.chars().next().unwrap() {
				'|' => rule.or = Some(rule.matches.len()),
				'"' => rule.matches.push(Match::Char(s.chars().nth(1).unwrap())),
				_ => rule.matches.push(Match::Rule(s.parse().unwrap())),
			}
		}
	}
	let mut rules2 = rules.clone();
	// 8: 42 | 42 8
	rules2[8] = Rule{
		matches: vec![Match::Rule(42), Match::Rule(42), Match::Rule(8),],
		or: Some(1),
	};
	// 11: 42 31 | 42 11 31
	rules2[11] = Rule{
		matches: vec![Match::Rule(42), Match::Rule(31), Match::Rule(42), Match::Rule(11), Match::Rule(31),],
		or: Some(2),
	};
	// println!("{:?}", rules);
	let mut res = 0;
	let mut res2 = 0;
	for l in lines {
		// println!("{:?}", check_rule(&l, &rules[0], &rules));
		let v = check_rule(&vec![&l], &rules[0], &rules);
		if v.iter().any(|s| s.len() == 0) {
			// println!("{:?}", v);
			res += 1;
		}
		let v = check_rule(&vec![&l], &rules2[0], &rules2);
		if v.iter().any(|s| s.len() == 0) {
			// println!("{:?}", v);
			res2 += 1;
		}
	}
	println!("{}", res);
	println!("{}", res2);
}

fn check_rule<'a, 'b>(s: &Vec<&'a str>, rule: &'b Rule, rules: &'b Vec<Rule>) -> Vec<&'a str> {
	let mut res = Vec::new();
	let mut start = 0;
	if let Some(c) = rule.or {
		if let Some(v) = rule.matches[0..c].iter().try_fold(s.clone(), |s, m| {
			let res = check_match(&s, m, rules);
			if res.len() > 0 {
				Some(res)
			} else {None}
		}) {
			res = v;
		}
		start = c;
	} 
	if let Some(mut r) = rule.matches[start..].iter().try_fold(s.clone(), |s, m| {
		let res = check_match(&s, m, rules);
		if res.len() > 0 {
			Some(res)
		} else {None}
	}) {
		res.append(&mut r);
	}
	res
}

fn check_match<'a, 'b>(s: &Vec<&'a str>, m: &'b Match, rules: &'b Vec<Rule>) -> Vec<&'a str> {
	match m {
		Match::Char(c) => {
			s.iter().filter_map(|s| {
				if *c == s.chars().next()? {
					Some(&s[1..])
				} else {
					None
				}
			}).collect()
		},
		Match::Rule(r) => {
			check_rule(s, &rules[*r], rules)
		},
	}
}