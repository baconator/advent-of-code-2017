extern crate regex;

use lib;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

struct Node<'a> {
	weight: u32,
	name: String,
	children: Vec<&'a Node<'a>>
}

struct NodeTree<'a> {
	nodes: Vec<Node<'a>>,
	root: &'a Node<'a>
}

enum WeightResult<'a> {
	Unbalanced(&'a Node<'a>),
	Weight(u32, &'a Node<'a>)
}

fn make_inputs() {
	let re = Regex::new(r"(^\w+) \(([0-9]+)\)(?: -> ((?:\w+, )*\w+))?").unwrap();
	let input_str = lib::load_inputs(&"input-7".to_string())
		.expect("");
	let name_to_node: HashMap<String, (u32, Vec<String>)> = HashMap::from_iter(input_str
		.lines()
		.map(|raw| { 
			let captures = re.captures(raw).unwrap();
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
			(name, (weight, children))
		} ));

	let mut nodes: HashMap<String, Node> = HashMap::from_iter(name_to_node.iter()
		.map(|(name, &(weight, ref child_names)): (&String, &(u32, Vec<String>))| { 
			(name.clone(), Node { weight: weight, name: name.clone(), children: Vec::new() })
		} ));
	for mut node in nodes.values_mut() {
		let child_names = &name_to_node[&node.name].1;
		node.children = child_names.iter().map(|child_name| { &nodes[child_name] } ).collect();
	}
}