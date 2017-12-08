use lib;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn puzzle1() {
	let mut blocks = lib::load_inputs(&"input-6".to_string())
		.expect("Couldn't load input.")
		.split("\t")
		.map(|input| { input.parse::<i32>().expect("Couldn't parse input.") } )
		.collect::<Vec<_>>();

	let mut seen_states: HashSet<Vec<i32>> = HashSet::new();

	while !seen_states.contains(&blocks) {
		seen_states.insert(blocks.clone());

		let mut max_index = 0;
		for (i, value) in blocks.iter().enumerate().skip(1) {
			if (blocks[i] > blocks[max_index]) || (blocks[i] == blocks[max_index] && i < max_index) {
				max_index = i;
			} 
		}

		let mut redistribute = blocks[max_index].clone();
		blocks[max_index] = 0;

		while redistribute > 0 {
			max_index = (max_index + 1) % blocks.len();
			blocks[max_index] += 1;
			redistribute -= 1;
		}
	}

	println!("{}", seen_states.len());
}

pub fn puzzle2() {
	let mut blocks = lib::load_inputs(&"input-6".to_string())
		.expect("Couldn't load input.")
		.split("\t")
		.map(|input| { input.parse::<i32>().expect("Couldn't parse input.") } )
		.collect::<Vec<_>>();

	let mut seen_states: HashMap<Vec<i32>, usize> = HashMap::new();
	let mut count = 0usize;
	let mut last_state = blocks.clone();

	while !seen_states.contains_key(&blocks) {
		seen_states.insert(blocks.clone(), count);
		count += 1;

		let mut max_index = 0;
		for (i, value) in blocks.iter().enumerate().skip(1) {
			if (blocks[i] > blocks[max_index]) || (blocks[i] == blocks[max_index] && i < max_index) {
				max_index = i;
			} 
		}

		let mut redistribute = blocks[max_index].clone();
		blocks[max_index] = 0;

		while redistribute > 0 {
			max_index = (max_index + 1) % blocks.len();
			blocks[max_index] += 1;
			redistribute -= 1;
		}
		last_state = blocks.clone();
	}

	println!("{}", count-seen_states[&last_state]);
}