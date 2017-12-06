use lib;
use std::cmp::max;
use std::cmp::min;

pub fn puzzle1() {
	let inputs_raw = lib::load_inputs(&"input-2".to_string()).expect("Failed to load inputs :'|");
	let table = inputs_raw.split("\r\n")
		.map(|r| { 
			r.split("\t")
			.map(|s| { s.parse::<u32>().expect("Couldn't parse int!") })
			.collect::<Vec<_>>() 
		} )
		.collect::<Vec<_>>();
	let checksum = table.iter().map(|row| {
		let largest = row.iter().fold(0, |acc, c| { max(acc, *c) });
		let smallest = row.iter().fold(u32::max_value(), |acc, c| { min(acc, *c) });
		largest - smallest
	}).sum::<u32>();
	println!("{}", checksum);
}

pub fn puzzle2() {
	let inputs_raw = lib::load_inputs(&"input-2".to_string()).expect("Failed to load inputs :'|");
	let table = inputs_raw.split("\r\n")
		.map(|r| { 
			r.split("\t")
			.map(|s| { s.parse::<u32>().expect("Couldn't parse int!") })
			.collect::<Vec<_>>() 
		} )
		.collect::<Vec<_>>();
	let checksum = table.iter().map(|row| {
		let mut left = 0;
		let mut right = 0;
		for outer in row.iter() {
			for inner in row.iter() {
				if outer != inner && max(outer, inner) % min(outer, inner) == 0 {
					left = *outer;
					right = *inner;
				}
			}
		}
		println!("l: {}, r: {}", left, right);

		max(left, right) / min(left, right)
	}).sum::<u32>();
	println!("{}", checksum);
}