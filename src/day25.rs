pub fn solve() {
	// let card = find(7, 12578151);
	// let door = find(7, 20152380);
	let card_c = 12578151;
	let door = find(7, 5051300);
	let key = crypt(card_c, door);
	println!("{}", key);
	assert_eq!(key, 296776);
}

fn crypt(mut val: u64, key: u64) -> u64 {
	val = val % 20201227;
	let mut res = 1;
	for _ in 0..key {
		res = (res * val) % 20201227;
	}
	res
}

fn find(mut clear: u64, target: u64) -> u64 {
	clear = clear % 20201227;
	let mut res = 1;
	let mut n = 0;
	while res != target {
		res = (res * clear) % 20201227;
		n += 1;
	}
	n
}