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

fn calc_perimeter_part2(map: &mut Vec<Vec<char>>) -> usize {
	let mut sum: usize = 0;
	let pos = map.iter().flatten().position(|c| *c == '#').unwrap();
	let line = pos / map.len();
	let column = pos % map.len();
	while map[line][column] != '*' {
		
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
