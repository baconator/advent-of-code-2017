use regex::Regex;

pub enum PuzzleError {
	Generic(String)
}

struct Group<'a> {
	garbage: Vec<Garbage<'a>>,
	groups: Vec<Group<'a>>
}

struct Garbage<'a> {
	contents: &'a str
}

const 

fn group(input: &str) {

}

fn garbage() {

}

pub fn puzzle1() -> Result<String, PuzzleError> {
	let inputs = include_str!("input-9");
	Ok("".to_string())
}