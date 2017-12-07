use lib;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn puzzle1() {
	let answer = lib::load_inputs(&"input-4".to_string())
		.expect("Couldn't load inputs :'|")
		.split("\n")
		.map(|row| { row.split(" ").collect::<Vec<_>>() } )
		.map(|row| { 
			let mut seen_words = HashSet::new();
			for word in row.iter() {
				if seen_words.contains(word) {
					println!("REJECTED {:?}", row);
					return 0;
				} else {
					seen_words.insert(word);
				}
			}
			println!("ACCEPTED {:?}", row);
			return 1;
		} ).sum::<u32>();
	println!("{}", answer);
}

pub fn puzzle2() {
	let answer = lib::load_inputs(&"input-4".to_string())
		.expect("Couldn't load inputs :'|")
		.split("\n")
		.map(|row| { row.split(" ").collect::<Vec<_>>() } )
		.map(|row| { 
			let mut seen_words: HashSet<String> = HashSet::new();
			for word_raw in row.iter() {
				let letters: HashSet<char> = HashSet::from_iter(word_raw.chars());
				let mut word_vec = letters.iter().collect::<Vec<_>>();
				word_vec.sort();
				let word = word_vec.into_iter().collect::<String>();
				if seen_words.contains(&word) {
					return 0;
				} else {
					seen_words.insert(word);
				}
			}
			return 1;
		} ).sum::<u32>();
	println!("{}", answer);
}