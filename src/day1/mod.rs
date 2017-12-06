use lib;

pub fn puzzle1() {
	let inputs_raw = lib::load_inputs(&"input-1".to_string()).expect("Failed to load inputs :'|");
	let inputs = inputs_raw.chars()
		.map(|i| { i.to_digit(10).ok_or(format!("Couldn't parse integer `{}`.", i)) } )
		.collect::<Result<Vec<_>, _>>()
		.expect("Couldn't parse integers.");
	
	let mut sum = 0;

	for i in 0..inputs.len() {
		let j = if i == inputs.len()-1 {
			0
		} else {
			i + 1
		};
		if inputs[i] == inputs[j] {
			sum += inputs[i];
		}
	}

	println!("{}", sum);
}

pub fn puzzle2() {
	let inputs_raw = lib::load_inputs(&"input-1".to_string()).expect("Failed to load inputs :'|");
	let inputs = inputs_raw.chars()
		.map(|i| { i.to_digit(10).ok_or(format!("Couldn't parse integer `{}`.", i)) } )
		.collect::<Result<Vec<_>, _>>()
		.expect("Couldn't parse integers.");
	
	let mut sum = 0;

	for i in 0..inputs.len() {
		let j = (i + inputs.len() / 2) % inputs.len();
		if inputs[i] == inputs[j] {
			sum += inputs[i];
		}
	}

	println!("{}", sum);
}