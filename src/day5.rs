use lib;
use std::collections::HashSet;

pub fn puzzle1() {
	let inputs_raw = lib::load_inputs(&"input-5".to_string())
		.expect("Couldn't load input.");
	let mut jump_offsets = inputs_raw
		.split("\n")
		.map(|s| { s.parse::<i32>().expect("Couldn't parse input as i32.") } )
		.collect::<Vec<_>>();

	let mut current_index = 0usize;
	let mut steps = 0;
	
	while current_index < jump_offsets.len() {
		jump_offsets[current_index] += 1;
		current_index = (current_index as i32 + jump_offsets[current_index]-1) as usize;
		steps += 1;
	}

	println!("{}", steps);
}

pub fn puzzle2() {
	let inputs_raw = lib::load_inputs(&"input-5".to_string())
		.expect("Couldn't load input.");
	let mut jump_offsets = inputs_raw
		.split("\n")
		.map(|s| { s.parse::<i32>().expect("Couldn't parse input as i32.") } )
		.collect::<Vec<_>>();

	let mut current_index = 0usize;
	let mut steps = 0;
	
	while current_index < jump_offsets.len() {
		let offset = jump_offsets[current_index].clone();
		if offset >= 3 {
			jump_offsets[current_index] -= 1;
		} else {
			jump_offsets[current_index] += 1;
		}
		current_index = (current_index as i32 + offset) as usize;
		steps += 1;
	}

	println!("{}", steps);
}