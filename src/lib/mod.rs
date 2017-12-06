use std;
use std::fs::File;
use std::io::Read;

pub fn load_inputs(filename: &String) -> Result<String, std::io::Error> {
	let mut f = File::open(format!("C:/Users/Bacon/Projects/advent-of-code/2017/src/{}", filename))?;
	let mut output = String::new();
	f.read_to_string(&mut output)?;
	return Ok(output);
}