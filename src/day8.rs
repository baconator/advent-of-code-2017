extern crate regex;
use self::regex::Regex;
use day8::regex::Captures;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug)]
pub enum PuzzleError {
	Generic(String),
	UnparseableCondition
}

enum Command {
	Increment,
	Decrement
}

enum Condition {
	LessThan,
	LessThanEqual,
	GreaterThan,
	GreaterThanEqual,
	Equal,
	NotEqual
}

impl <'a> From<&'a str> for Condition {
	fn from(input: &'a str) -> Condition {
		match input {
			"<" => Condition::LessThan,
			"<=" => Condition::LessThanEqual,
			">" => Condition::GreaterThan,
			">=" => Condition::GreaterThanEqual,
			"==" => Condition::Equal,
			"!=" => Condition::NotEqual,
			_ => panic!("Couldn't parse condition.")
		}
	}
}

impl <'a> From<&'a str> for Command {
	fn from(input: &'a str) -> Command {
		match input {
			"inc" => Command::Increment,
			"dec" => Command::Decrement,
			_ => panic!("Couldn't parse command.")
		}
	}
}

struct Instruction<'a> {
	target: &'a str,
	command: Command,
	value: i32,
	conditional_target: &'a str,
	condition: Condition,
	conditional_value: i32
}

struct State<'a> {
	registers: HashMap<&'a str, i32>,
	max_value: i32
}

impl <'a> State<'a> {
	fn new() -> State<'a> {
		State { registers: HashMap::new(), max_value: <i32>::min_value() }
	}
}

impl Condition {
	fn evaluate(&self, target: &i32, test: &i32) -> bool {
		match self {
			&Condition::LessThan => target < test,
			&Condition::LessThanEqual => target <= test,
			&Condition::GreaterThan => target > test,
			&Condition::GreaterThanEqual => target >= test,
			&Condition::Equal => target == test,
			&Condition::NotEqual => target != test
		}
	}
}

impl Command {
	fn execute<'a>(&self, target: &i32, value: &i32) -> i32 {
		match self {
			&Command::Increment => target + value,
			&Command::Decrement => target - value
		}
	}
}

fn make_state<'a>() -> Result<State<'a>, PuzzleError> {
	let matcher = Regex::new(r"^(\w+) (inc|dec) (-?[0-9]+) if (\w+) (<|<=|>|>=|==|!=) (-?[0-9]+)$").map_err(|e| { PuzzleError::Generic(e.to_string()) } )?; 
	let instructions = include_str!("input-8")
		.lines()
		.map(|line| { 
			let results: Captures = matcher.captures(line).unwrap();
			Instruction { 
				target: &results.get(1).unwrap().as_str(), 
				command: results[2].into(), 
				value: results[3].parse::<i32>().unwrap(), 
				conditional_target: &results.get(4).unwrap().as_str(), 
				condition: results[5].into(), 
				conditional_value: results[6].parse::<i32>().unwrap()
			}
		} );

	Ok(instructions.fold(State::new(), |mut state, instruction| {
		{
			state.registers.entry(instruction.target).or_insert(0);
			state.registers.entry(instruction.conditional_target).or_insert(0);
			let should_execute = instruction.condition.evaluate(&state.registers[instruction.conditional_target], &instruction.conditional_value);
			if should_execute {
				let new_value = instruction.command.execute(&state.registers[instruction.target], &instruction.value);
				state.max_value = max(state.max_value, new_value);
				*state.registers.entry(instruction.target).or_insert(0) = new_value;
			}
		}
		state
	} ))
}

pub fn puzzle1() -> Result<String, PuzzleError> {
	let final_state = make_state()?;

	Ok(final_state.registers.values().max().unwrap().to_string())
}

pub fn puzzle2() -> Result<String, PuzzleError> {
	let final_state = make_state()?;

	Ok(final_state.max_value.to_string())
}