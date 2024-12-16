pub fn solve(input: String) -> (usize, usize) {
	let (map, move_sequence) = parse_input(input);
	let part1 = solve_part1(map.clone(), move_sequence.clone());
	let part2 = solve_part2(map, move_sequence);
    (part1, part2) 
}

fn parse_input(input: String) -> (Vec<Vec<char>>, String) {
	let mut map: Vec<Vec<char>> = Vec::new();
	let mut move_sequence = String::new();
	let mut i = 0;
	while input.lines().nth(i).unwrap() != "" {
		map.push(input.lines().nth(i).unwrap().chars().collect());
		i += 1;
	}
	i += 1;
	while i < input.lines().count() {
		move_sequence.push_str(&input.lines().nth(i).unwrap());
		i += 1;
	}
	(map, move_sequence)
}

fn 	move_dir(map: &mut Vec<Vec<char>>, line: usize, column: usize, direction: char) -> bool {
	if map[line][column] == '.' {
		return true;
	}
	if map[line][column] == '#' {
		return false;
	}
	let mut new_line = line;
	let mut new_column = column;
	match direction {
		'^' => new_line -= 1,
		'>' => new_column += 1,
		'v' => new_line += 1,
		'<' => new_column -= 1,
		_ => println!("this should never happen"),
	}
	if map[line][column] == 'O' {
		if move_dir(map, new_line, new_column, direction) {
			map[line][column] = map[new_line][new_column];
			map[new_line][new_column] = 'O';
			return true;
		}
		return false;
	}
	if map[line][column] == '@' && ((map[new_line][new_column] != '[' && map[new_line][new_column] != ']') || direction == '>' || direction == '<') {
		if move_dir(map, new_line, new_column, direction) {
			map[line][column] = map[new_line][new_column];
			map[new_line][new_column] = '@';
			return true;
		}
		return false;
	}
	if map[line][column] == '@' && !((map[new_line][new_column] != '[' && map[new_line][new_column] != ']') || direction == '>' || direction == '<') {
		return move_dir(map, new_line, new_column, direction);
	}
	if (map[line][column] == '[' || map[line][column] == ']') && (direction == '<' || direction == '>') {
		if move_dir(map, new_line, new_column, direction) {
			let old = map[line][column];
			map[line][column] = map[new_line][new_column];
			map[new_line][new_column] = old;
			return true;
		}
		return false;
	}
	let mut allowed = false;
	if map[line][column] == '[' && (direction == '^' || direction == 'v') {
		allowed = move_dir(map, new_line, new_column, direction) && move_dir(map, new_line, new_column+1, direction);
	}
	if map[line][column] == ']' && (direction == '^' || direction == 'v') {
		allowed = move_dir(map, new_line, new_column, direction) && move_dir(map, new_line, new_column-1, direction);
	}
	allowed
}

fn make_move(map: &mut Vec<Vec<char>>, line: usize, column: usize, direction: char) {
	if map[line][column] == '.' {
		return;
	}
	if map[line][column] == '#' {
		println!("encountered wall in make_move!!!");
		return;
	}
	let mut new_line = line;
	match direction {
		'^' => new_line -= 1,
		'v' => new_line += 1,
		_ => println!("this should never happen"),
	}
	if map[line][column] == '@' {
		make_move(map, new_line, column, direction);
		map[line][column] = map[new_line][column];
		map[new_line][column] = '@';
	}
	if map[line][column] == '[' {
		make_move(map, new_line, column, direction);
		make_move(map, new_line, column+1, direction);
		map[line][column] = map[new_line][column];
		map[line][column+1] = map[new_line][column+1];
		map[new_line][column] = '[';
		map[new_line][column+1] = ']';
	}
	if map[line][column] == ']' {
		make_move(map, new_line, column, direction);
		make_move(map, new_line, column-1, direction);
		map[line][column] = map[new_line][column];
		map[line][column-1] = map[new_line][column-1];
		map[new_line][column] = ']';
		map[new_line][column-1] = '[';
	}
}

fn do_movement(mut map: &mut Vec<Vec<char>>, move_sequence: String) {
	let mut line = map.iter().position(|line| line.contains(&'@')).unwrap();
	let mut column = map[line].iter().position(|c| *c == '@').unwrap();
	for direction in move_sequence.chars() {
		if move_dir(&mut map, line, column, direction) {
			if map[line][column] == '@' {
				make_move(&mut map, line, column, direction);
			}
			match direction {
				'^' => line -= 1,
				'>' => column += 1,
				'v' => line += 1,
				'<' => column -= 1,
				_ => println!("this should never happen"),
			}
		}
	}
}

fn resize(map: &mut Vec<Vec<char>>) {
	let map_clone = map.clone();
	map.clear();
	for line in map_clone {
		let mut new_line: Vec<char> = Vec::new();
		for char in line {
			match char {
				'#' => {
					new_line.push('#');
					new_line.push('#');
				},
				'O' => {
					new_line.push('[');
					new_line.push(']');
				},
				'.' => {
					new_line.push('.');
					new_line.push('.');
				},
				'@' => {
					new_line.push('@');
					new_line.push('.');
				},
				_ => println!("this should never happen"),
			}
		}
		map.push(new_line);
	}
}

fn solve_part1(mut map: Vec<Vec<char>>, move_sequence: String) -> usize {
	do_movement(&mut map, move_sequence);
	let mut sum: usize = 0;
	for line in 0..map.len() {
		for column in 0..map[0].len() {
			if map[line][column] == 'O' {
				sum += line * 100 + column;
			}
		}
	}
	sum
}

fn solve_part2(mut map: Vec<Vec<char>>, move_sequence: String) -> usize {
	resize(&mut map);
	do_movement(&mut map, move_sequence);
	for line in &map {
		println!("{}", line.iter().collect::<String>());
	}
	let mut sum: usize = 0;
	for line in 0..map.len() {
		for column in 0..map[0].len() {
			if map[line][column] == '[' {
				sum += line * 100 + column;
			}
		}
	}
	sum
}









