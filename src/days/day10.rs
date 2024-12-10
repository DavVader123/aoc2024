use std::collections::HashSet;

pub fn solve(input: String) -> (usize, usize) {
	let parsed: Vec<Vec<u8>> = input.lines().map(|line| line.chars().map(|num| num.to_digit(10).unwrap() as u8).collect()).collect();
	let part1 = solve_part1(parsed.clone()); 
	let part2 = solve_part2(parsed); 
    (part1, part2) 
}

fn reachable_nines(map: &Vec<Vec<u8>>, line: usize, column: usize) -> HashSet<(usize, usize)> {
	if map[line][column] == 9 {
		let mut reachable:HashSet<(usize,usize)> = HashSet::new();
		reachable.insert((line, column));
		return reachable;
	}
	let mut reachable: HashSet<(usize, usize)> = HashSet::new();
	if line > 0 && map[line - 1][column] == map[line][column]+1 {
		reachable.extend(reachable_nines(map, line - 1, column).iter());
	}
	if column > 0 && map[line][column - 1] == map[line][column]+1 {
		reachable.extend(reachable_nines(map, line, column - 1).iter());
	}
	if line < map.len() - 1 && map[line + 1][column] == map[line][column]+1 {
		reachable.extend(reachable_nines(map, line + 1, column).iter());
	}
	if column < map[0].len() - 1 && map[line][column + 1] == map[line][column]+1 {
		reachable.extend(reachable_nines(map, line, column + 1).iter());
	}
	reachable
}

fn rating(map: &Vec<Vec<u8>>, line: usize, column: usize) -> usize {
	if map[line][column] == 9 {
		return 1;
	}
	let mut sum: usize = 0;
	if line > 0 && map[line - 1][column] == map[line][column]+1 {
		sum += rating(map, line - 1, column);
	}
	if column > 0 && map[line][column - 1] == map[line][column]+1 {
		sum += rating(map, line, column - 1);
	}
	if line < map.len() - 1 && map[line + 1][column] == map[line][column]+1 {
		sum += rating(map, line + 1, column);
	}
	if column < map[0].len() - 1 && map[line][column + 1] == map[line][column]+1 {
		sum += rating(map, line, column + 1);
	}
	sum
}
 
fn solve_part1(map: Vec<Vec<u8>>) -> usize {
	let mut sum: usize = 0;
	for line in map.iter().enumerate() {
		for column in line.1.iter().enumerate() {
			if *column.1 == 0 {
				sum += reachable_nines(&map, line.0, column.0).len();
			}
		}
	}
	sum
} 
 
fn solve_part2(map: Vec<Vec<u8>>) -> usize {
	let mut sum: usize = 0;
	for line in map.iter().enumerate() {
		for column in line.1.iter().enumerate() {
			if *column.1 == 0 {
				sum += rating(&map, line.0, column.0);
			}
		}
	}
	sum
} 
