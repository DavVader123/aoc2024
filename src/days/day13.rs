use std::ops::{Add, Mul};

pub fn solve(input: String) -> (usize, usize) { 
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
}

#[derive(Debug, Clone)]
pub struct ClawMachine {
	pub a: (usize, usize),
	pub b: (usize, usize),
	pub goal: (usize, usize),
}

pub struct Axis {
	pub a: f64,
	pub b: f64,
	pub goal: f64,
}

impl Mul<f64> for Axis {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		Self{a: self.a * rhs, b: self.b * rhs, goal: self.goal * rhs}
	}
}

impl Add for Axis {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self{a: self.a + rhs.a, b: self.b + rhs.b, goal: self.goal + rhs.goal}
	}
}

fn parse_input(input: String) -> Vec<ClawMachine> {
	let mut result: Vec<ClawMachine> = Vec::new();
	let lines = input.lines().map(|line| line.to_string()).collect::<Vec<String>>();
	let mut i: usize = 0;
	while i < lines.len() {
		let mut machine = ClawMachine{
			a: (0, 0),
			b: (0, 0),
			goal: (0, 0),
		};
		let values = lines[i]
			.split(",")
			.map(|val| {val.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap()})
			.collect::<Vec<usize>>();
		assert_eq!(values.len(), 2);
		machine.a = (values[0], values[1]);
		i += 1;
		let values = lines[i]
			.split(",")
			.map(|val| {val.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap()})
			.collect::<Vec<usize>>();
		assert_eq!(values.len(), 2);
		machine.b = (values[0], values[1]);
		i += 1;
		let values = lines[i]
			.split(",")
			.map(|val| {val.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap()})
			.collect::<Vec<usize>>();
		assert_eq!(values.len(), 2);
		machine.goal = (values[0], values[1]);
		result.push(machine);
		i += 2;
	}
	result
}
 
fn solve_part1(input: String) -> usize {
	let test = parse_input(input);
	println!("{:?}", test);
	0
} 
 
fn solve_part2(input: String) -> usize { 
0 
} 
