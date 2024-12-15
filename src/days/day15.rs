pub fn solve(input: String) -> (usize, usize) {
	let (map, move_sequence) = parse_input(input);
	let part1 = 0;//solve_part1(map.clone(), move_sequence.clone());
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

fn 	move_dir(map: &mut Vec<Vec<char>>, mut line: usize, mut column: usize, direction: char) -> bool {

	let old_line = line;
	let old_column = column;
	match direction {
		'^' => line -= 1,
		'>' => column += 1,
		'v' => line += 1,
		'<' => column -= 1,
		_ => println!("this should never happen"),
	}
	if map[line][column] == '.' {
		if (direction == '^' || direction == 'v') && (map[old_line][old_column] == '[' && map[line][column+1] == '#' || map[old_line][old_column] == ']' && map[line][column-1] == '#') {
			return false;
		}
		map[line][column] = map[old_line][old_column];
		map[old_line][old_column] = '.';
		return true;
	}
	if map[line][column] == '#' {
		return false;
	}
	if map[line][column] == 'O' {
		if move_dir(map, line, column, direction) {
			if map[old_line][old_column] == 'O' {
				map[line][column] = map[old_line][old_column];
				map[old_line][old_column] = 'O';
			} else if map[old_line][old_column] == '@' {
				map[line][column] = map[old_line][old_column];
				map[old_line][old_column] = '.';
			} else {
				println!("This should never happen");
			}
			return true;
		}
		return false;
	}
	if map[line][column] == '[' || map[line][column] == ']' {
		if direction == '>' || direction == '<' {
			if move_dir(map, line, column, direction) {
				if map[old_line][old_column] == '[' {
					map[line][column] = map[old_line][old_column];
					map[old_line][old_column] = '[';
				} else if map[old_line][old_column] == ']' {
					map[line][column] = map[old_line][old_column];
					map[old_line][old_column] = ']';
				} else if map[old_line][old_column] == '@' {
					map[line][column] = map[old_line][old_column];
					map[old_line][old_column] = '.';
				} else {
					println!("This should never happen");
				}
			}
		} else {
			if map[line][column] == '[' {
				if move_dir(map, line, column, direction) && move_dir(map, line, column+1, direction) {
					if map[old_line][old_column] == '[' {
						map[line][column] = map[old_line][old_column];
						map[old_line][old_column] = '.';
					} else if map[old_line][old_column] == ']' {
						map[line][column] = map[old_line][old_column];
						map[old_line][old_column] = '.';
					} else if map[old_line][old_column] == '@' {
						map[line][column] = map[old_line][old_column];
						map[old_line][old_column] = '.';
					} else {
						println!("This should never happen");
						return false;
					}
				}
			} else if map[line][column] == ']' {
				if move_dir(map, line, column, direction) && move_dir(map, line, column-1, direction) {
					if map[old_line][old_column] == '[' {
						map[line][column] = map[old_line][old_column];
						map[old_line][old_column] = '.';
					} else if map[old_line][old_column] == ']' {
						map[line][column] = map[old_line][old_column];
						map[old_line][old_column] = '.';
					} else if map[old_line][old_column] == '@' || map[old_line][old_column] == '.' {
						map[line][column] = map[old_line][old_column];
						map[old_line][old_column] = '.';
					} else {
						println!("This should never happen");
						return false;
					}
				}
			}
		}
		return true;
	}
	false
}

fn do_movement(mut map: &mut Vec<Vec<char>>, move_sequence: String) {
	let mut line = map.iter().position(|line| line.contains(&'@')).unwrap();
	let mut column = map[line].iter().position(|c| *c == '@').unwrap();
	for direction in move_sequence.chars() {
		if move_dir(&mut map, line, column, direction) {
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
	0
}









