//noinspection DuplicatedCode
pub fn solve(input: String) -> (usize, usize) { 
	let parsed_input = input.lines().collect::<Vec<&str>>().iter().map(|&x| x.to_string().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
	let part1 = solve_part1(parsed_input.clone()); 
	let part2 = solve_part2(parsed_input.clone()); 
    (part1, part2) 
}

fn predict_path(map: &mut Vec<Vec<char>>) -> bool{
	let mut line = map.iter().position(|x| x.contains(&'^')).unwrap();
	let mut column = map[line].iter().position(|&x| x == '^').unwrap();

	loop {
		match map[line][column] {
			'^' => {
				if line == 0 {
					map[line][column] = 'X';
					break;
				}
				if map[line-1][column] == '#' || map[line-1][column] == 'O' || map[line-1][column] == 'S' || map[line-1][column] == 'W' {
					map[line-1][column] = 'N';
					map[line][column] = '>';
				} else if map[line-1][column] == 'N' {
					return false;
				} else {
					map[line-1][column] = '^';
					map[line][column] = 'X';
					line -= 1;
				}
			},
			'>' => {
				if column >= map[0].len()-1 {
					map[line][column] = 'X';
					break;
				}
				if map[line][column+1] == '#' || map[line][column+1] == 'S' || map[line][column+1] == 'W' || map[line][column+1] == 'N' {
					map[line][column+1] = 'O';
					map[line][column] = 'v';
				} else if map[line][column+1] == 'O' {
					return false;
				} else {
					map[line][column+1] = '>';
					map[line][column] = 'X';
					column += 1;
				}
			},
			'v' => {
				if line >= map.len()-1 {
					map[line][column] = 'X';
					break;
				}
				if map[line+1][column] == '#' || map[line+1][column] == 'W' || map[line+1][column] == 'N' || map[line+1][column] == 'O' {
					map[line+1][column] = 'S';
					map[line][column] = '<';
				} else if map[line+1][column] == 'S' {
					return false;
				} else {
					map[line+1][column] = 'v';
					map[line][column] = 'X';
					line += 1;
				}
			},
			'<' => {
				if column == 0 {
					map[line][column] = 'X';
					break;
				}
				if map[line][column-1] == '#' || map[line][column-1] == 'N' || map[line][column-1] == 'O' || map[line][column-1] == 'S' {
					map[line][column-1] = 'W';
					map[line][column] = '^';
				} else if map[line][column-1] == 'W' {
					return false;
				} else { 
					map[line][column-1] = '<';
					map[line][column] = 'X';
					column -= 1;
				}
			},
			_ => panic!("Something went wrong!")
		}
	}
	true
}

fn check_loop(mut map: Vec<Vec<char>>, line: usize, column: usize) -> bool {
	map[line][column] = '#';
	!predict_path(&mut map)
}
 
fn solve_part1(input: Vec<Vec<char>>) -> usize {
	let mut map = input.clone();
	predict_path(&mut map);
	map.iter().flatten().filter(|&&x| x == 'X').count()
} 
 
fn solve_part2(input: Vec<Vec<char>>) -> usize {
	let mut sum: usize = 0;
	for i in 0..input.len() {
		for j in 0..input[0].len() {
			if input[i][j] == '.' {
				sum += check_loop(input.clone(), i, j) as usize;
			}
		}
	}
	sum
} 
