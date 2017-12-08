extern crate regex;

use lib;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct Node {
	weight: u32,
	children: Vec<String>
}

enum WeightResult<'a> {
	Unbalanced(&'a Node),
	Weight(u32, &'a Node)
}

fn check_weight<'a>(node: &'a Node, inputs: &'a HashMap<String, Node>) -> WeightResult<'a> {
	if node.children.is_empty() {
		WeightResult::Weight(node.weight, node)
	} else {
		let child_weights = node.children
			.iter()
			.map(|child| { check_weight(&inputs[child], inputs) } );

		let mut children_by_weight: HashMap<u32, HashSet<&Node>> = HashMap::new();

		for child_weight in child_weights {
			match child_weight {
				WeightResult::Unbalanced(_) => return child_weight,
				WeightResult::Weight(w, n) => {
					let weight_class = children_by_weight.entry(w).or_insert(HashSet::new());
					weight_class.insert(n);

					// put check here.
				} 
			}
		}

		WeightResult::Weight(node.weight, node)
	}
}

pub fn puzzle1() {
	let re = Regex::new(r"(^\w+) \(([0-9]+)\)(?: -> ((?:\w+, )*\w+))?").unwrap();
	let input_str = lib::load_inputs(&"input-7".to_string())
		.expect("");
	let inputs: HashMap<String, _> = HashMap::from_iter(input_str
		.split("\n")
		.filter_map(|raw| { 
			for captures in re.captures(raw) {
				let name = captures.get(1).unwrap().as_str().to_string();
				let weight = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
				let children = captures.get(3)
					.map_or_else(|| { Vec::new() }, |c| { 
						c.as_str()
							.to_string()
							.split(", ")
							.map(|s| { s.to_string() } )
							.collect::<Vec<String>>() 
					});
				return Some((name, Node { weight: weight, children: children } ));
			} 
			None
		} ));
	let mut parents: HashMap<String, HashSet<String>> = HashMap::new();

	for (node, value) in inputs.iter() {
		for child in value.children.iter() {
			let ps = parents.entry(child.clone()).or_insert(HashSet::new());
			ps.insert(node.clone());
		}
	}

	for (node, ps) in parents.iter() {
		if ps.len() == 0 {
			println!("{}", node);
		}
	}
	
	let parent_keys: HashSet<&String> = HashSet::from_iter(parents.keys());
	for (node, _) in inputs.iter() {
		if !parent_keys.contains(node) {
			println!("{}", node);
		}
	}
}