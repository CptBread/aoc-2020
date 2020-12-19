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
	// println!("{:?}", rules);
	let mut res = 0;
	for l in lines {
		// println!("{:?}", check_rule(&l, &rules[0], &rules));
		if let Some("") = check_rule(&l, &rules[0], &rules) {
			res += 1;
		}
	}
	println!("{}", res);
}

fn check_rule<'a, 'b>(s: &'a str, rule: &'b Rule, rules: &'b Vec<Rule>) -> Option<&'a str> {
	if let Some(c) = rule.or {
		for chunk in rule.matches.chunks(c) {
			let res = chunk.iter().try_fold(s, |s, m| {
				let res = check_match(s, m, rules);
				res
			});
			if res.is_some() {
				return res;
			}
		}
		None
	} else 
	{
		let res = rule.matches.iter().try_fold(s, |s, m| {
			let res = check_match(s, m, rules);
			res
		});
		res
	}
}

fn check_match<'a, 'b>(s: &'a str, m: &'b Match, rules: &'b Vec<Rule>) -> Option<&'a str> {
	match m {
		Match::Char(c) => {
			if *c == s.chars().next()? {
				Some(&s[1..])
			} else {
				None
			}
		},
		Match::Rule(r) => {
			check_rule(s, &rules[*r], rules)
		},
	}
}