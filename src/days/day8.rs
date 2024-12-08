use std::collections::HashSet;
use std::ops::{Add, Sub, Mul};

pub fn solve(input: String) -> (usize, usize) {
	let field = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
	let mut antennas: HashSet<(char, Point)> = HashSet::new();
	for y in 0..field.len() {
		for x in 0..field[0].len() {
			if field[y][x] != '.' {
				antennas.insert((field[y][x], Point{x: x as i32,y: y as i32}));
			}
		}
	}
	let part1 = solve_part1(antennas.clone(), (field[0].len()-1) as i32, (field.len()-1) as i32); 
	let part2 = solve_part2(antennas.clone(), (field[0].len()-1) as i32, (field.len()-1) as i32); 
    (part1, part2) 
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Point {
	pub x: i32,
	pub y: i32,
}

impl Add for &Point {
	type Output = Point;
	fn add(self, other: &Point) -> Point {
		Point{x: self.x + other.x, y: self.y + other.y}
	}
}

impl Sub for &Point {
	type Output = Point;
	fn sub(self, other: &Point) -> Point {
		Point{x: self.x - other.x, y: self.y - other.y}
	}
}

impl Mul<i32> for &Point {
	type Output = Point;
	fn mul(self, other: i32) -> Point {
		Point{x: self.x * other, y: self.y * other}
	}
}

fn solve_part1(antennas: HashSet<(char, Point)>, max_x: i32, max_y: i32) -> usize {
	let mut antinodes: HashSet<Point> = HashSet::new();
	for node in &antennas {
		for other in antennas.iter().filter(|n| n.0 == node.0 && n.1 != node.1 ) {
			let antinode = &node.1 + &(&node.1 - &other.1);
			if antinode.x <= max_x && antinode.x >= 0 && antinode.y <= max_y && antinode.y >= 0 {
				antinodes.insert(antinode);
			}
		}
	}
	antinodes.len()
} 

fn solve_part2(antennas: HashSet<(char, Point)>, max_x: i32, max_y: i32) -> usize {
	let mut antinodes: HashSet<Point> = HashSet::new();
	for node in &antennas {
		for other in antennas.iter().filter(|n| n.0 == node.0 && n.1 != node.1 ) {
			let dir = &(&node.1 - &other.1);
			let mut i = 0;
			loop {
				let antinode = &node.1 + &(dir * i);
				if antinode.x > max_x || antinode.x < 0 || antinode.y > max_y || antinode.y < 0 {
					break;
				}
				antinodes.insert(antinode);
				i += 1;
			}
		}
	}
	antinodes.len()
}
