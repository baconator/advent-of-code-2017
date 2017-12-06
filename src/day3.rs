use std::collections::HashMap;
use std::cmp;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const NEIGHBOURS: [(i32, i32); 8] = [(1, 0), (0, 1), (-1, 0), (0, -1), (-1, 1), (1, 1), (-1, -1), (1, -1)];

struct SpiralIterator {
	state: u32,
	count: u32,
	row: u32,
	position: (i32, i32)
}

impl Iterator for SpiralIterator {
	type Item = (i32, i32);
	fn next(&mut self) -> Option<(i32, i32)> {
		self.count += 1;
		let direction = match self.state {
			0 => {
				self.state = 1;
				self.count = 0;
				DIRECTIONS[0]
			},
			1 => {
				if self.count == 2*self.row-1 {
					self.state = 2;
					self.count = 0;
				};
				DIRECTIONS[1]
			},
			2 => {
				if self.count == 2*self.row {
					self.state = 3;
					self.count = 0;
				};
				DIRECTIONS[2]
			},
			3 => {
				if self.count == 2*self.row {
					self.state = 4;
					self.count = 0;
				};
				DIRECTIONS[3]
			},
			4 => {
				if self.count == 2*self.row {
					self.state = 0;
					self.count = 0;
					self.row += 1;
				};
				DIRECTIONS[0]
			},
			_ => panic!("No state matchable.")
		};
		let output = self.position.clone();
		self.position.0 += direction.0;
		self.position.1 += direction.1;
		Some(output)
	}
}

impl SpiralIterator {
	fn new() -> SpiralIterator {
		SpiralIterator { state: 0, count: 0, row: 1, position: (0, 0) }
	}
}

struct SummingSpiralIterator {
	wrapped_iterator: Box<Iterator<Item = (i32, i32)>>,
	cells: HashMap<(i32, i32), i32>
}

impl Iterator for SummingSpiralIterator {
	type Item = i32;
	fn next(&mut self) -> Option<i32> {
		let position = &self.wrapped_iterator.next().expect("Unlimited power");
		let neighbours = NEIGHBOURS.iter()
			.filter_map(|key| { self.cells.get(&(position.0 + key.0, position.1 + key.1)) } )
			.sum::<i32>();
		let value = cmp::max(1, neighbours);
		self.cells.insert(*position, value);
		Some(value)
	}
}

impl SummingSpiralIterator {
	fn new() -> SummingSpiralIterator {
		SummingSpiralIterator { wrapped_iterator: Box::new(SpiralIterator::new()), cells: HashMap::new() }
	}
}

pub fn puzzle1() {
	for (x, y) in SpiralIterator::new().skip(347990).take(1) {
		println!("{} {}", x, y);
		println!("{}", x.abs() + y.abs());
	}
}

pub fn puzzle2() {
	for (i, n) in SummingSpiralIterator::new().enumerate() {
		if n > 347991 {
			println!("{}: {}", i, n);
			break;
		}
	}
}