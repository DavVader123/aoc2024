pub fn solve(input: String) -> (usize, usize) { 
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
}

fn paint_area(map: &mut Vec<Vec<char>>, line: usize, column: usize) {
	let plant = map[line][column];
	map[line][column] = '#';
	if line > 0 && map[line - 1][column] == plant {
		paint_area(map, line - 1, column);
	}
	if line < map.len() - 1 && map[line + 1][column] == plant {
		paint_area(map, line + 1, column);
	}
	if column > 0 && map[line][column - 1] == plant {
		paint_area(map, line, column - 1);
	}
	if column < map[0].len() - 1 && map[line][column + 1] == plant {
		paint_area(map, line, column + 1);
	}
}

fn calc_area(map: &mut Vec<Vec<char>>, line: usize, column: usize) -> usize {
	paint_area(map, line, column);
	map.iter().flatten().filter(|c| **c == '#').count()
}

fn value(map: &mut Vec<Vec<char>>, line: usize, column: usize) -> usize {
	let mut sum: usize = 0;
	if line == 0 || map[line - 1][column] != '#' {
		sum += 1;
	}
	if line == map.len() - 1 || map[line + 1][column] != '#' {
		sum += 1;
	}
	if column == 0 || map[line][column - 1] != '#' {
		sum += 1;
	}
	if column == map[0].len() - 1 || map[line][column + 1] != '#' {
		sum += 1;
	}
	sum
}

fn calc_perimeter(map: &mut Vec<Vec<char>>) -> usize {
	let mut sum: usize = 0;
	for line in 0..map.len() {
		for column in 0..map[0].len() {
			if map[line][column] == '#' {
				sum += value(map, line, column);
			}
		}
	}
	map.iter_mut().flatten().filter(|c| **c == '#').for_each(|c| *c = '*');
	sum
}

fn calc_cost(map: &mut Vec<Vec<char>>, line: usize, column: usize) -> usize {
	calc_area(map, line, column) * calc_perimeter(map)
}

fn value_part2(map: &mut Vec<Vec<char>>, line: usize, column: usize) -> usize {
	let mut sum: usize = 0;
	let up = line > 0 && map[line - 1][column] == '#';
	let right = column < map[0].len() - 1 && map[line][column + 1] == '#';
	let down = line < map.len() - 1 && map[line + 1][column] == '#';
	let left = column > 0 && map[line][column - 1] == '#';
	let upright = line > 0 && column < map[0].len()-1 && map[line - 1][column + 1] == '#';
	let upleft = line > 0 && column > 0 && map[line - 1][column - 1] == '#';
	let downright = line < map.len() - 1 && column < map[0].len()-1 && map[line + 1][column + 1] == '#';
	let downleft = line < map.len() - 1 && column > 0 && map[line + 1][column - 1] == '#';
	if up && down && !right && !left || right && left && !up && !down {
		return 0;
	}
	if !left && !right && !up && !down {
		return 4;
	}
	if up && !right && !left && !down || right && !up && !left && !down || down && !up && !left && !right || left && !up && !right && !down {
		return 2;
	}
	if up && right {
		if !upright {
			sum += 1;
		}
		if !down && !left {
			sum += 1;
		}
	}
	if right && down {
		if !downright {
			sum += 1;
		}
		if !up && !left {
			sum += 1;
		}
	}
	if down && left {
		if !downleft {
			sum += 1;
		}
		if !up && !right {
			sum += 1;
		}
	}
	if left && up {
		if !upleft {
			sum += 1;
		}
		if !down && !right {
			sum += 1;
		}
	}
	sum
}

fn calc_perimeter_part2(map: &mut Vec<Vec<char>>) -> usize {
	let mut sum: usize = 0;
	for line in 0..map.len() {
		for column in 0..map[0].len() {
			if map[line][column] == '#' {
				sum += value_part2(map, line, column);
			}
		}
	}
	map.iter_mut().flatten().filter(|c| **c == '#').for_each(|c| *c = '*');
	sum
}

fn calc_cost_part2(map: &mut Vec<Vec<char>>, line: usize, column: usize) -> usize {
	calc_area(map, line, column) * calc_perimeter_part2(map)
}
 
fn solve_part1(input: String) -> usize {
	let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	let mut sum: usize = 0;
	for line in 0..map.len() {
		for column in 0..map[0].len() {
			if map[line][column] != '*' {
				sum += calc_cost(&mut map, line, column);
			}
		}
	}
	sum
} 
 
fn solve_part2(input: String) -> usize {
	let mut map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
	let mut sum: usize = 0;
	for line in 0..map.len() {
		for column in 0..map[0].len() {
			if map[line][column] != '*' {
				sum += calc_cost_part2(&mut map, line, column);
			}
		}
	}
	sum
} 
