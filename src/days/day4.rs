pub fn solve(input: String) -> (usize, usize) { 
	let parsed_input = input.lines().collect::<Vec<&str>>().iter().map(|&x| x.to_string().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
	let part1 = solve_part1(parsed_input.clone()); 
	let part2 = solve_part2(parsed_input.clone()); 
    (part1, part2) 
}

fn check_out_of_bounds_xmas(lines: usize, columns: usize, mut position: (i32, i32), direction: (i32, i32)) -> bool {
	position.0 += 3*direction.0;
	position.1 += 3*direction.1;
	position.0 < 0 || position.1 < 0 || position.0 >= lines as i32 || position.1 >= columns as i32
}

fn check_out_of_bounds_x_mas(lines: usize, columns: usize, mut position: (i32, i32)) -> bool {
	position.0 <=0 || position.1 <= 0 || position.0 >= (lines - 1) as i32 || position.1 >= (columns-1) as i32
}

fn check_direction(word_search: &Vec<Vec<char>>, mut position: (i32, i32), direction: (i32, i32)) -> bool {
	if check_out_of_bounds_xmas(word_search.len(), word_search[0].len(),position,direction) {
		return false;
	}
	let letters = ['M','A','S'];
	let mut found = true;
	for i in 0..3 {
		position.0 += direction.0;
		position.1 += direction.1;
		
		if word_search[position.0 as usize][position.1 as usize] != letters[i] {
			found = false;
			break;
		}
	}
	found
}

fn check_xmas(word_search: &Vec<Vec<char>>, line: usize, column: usize) -> usize {
	let mut num_xmas:usize = 0;
	for i in 0..8{
		num_xmas += match i {
			0 => check_direction(&word_search, (line as i32,column as i32), (0,1)),	// right
			1 => check_direction(&word_search, (line as i32,column as i32), (1,1)),	// right down
			2 => check_direction(&word_search, (line as i32,column as i32), (1,0)),	// down
			3 => check_direction(&word_search, (line as i32,column as i32), (1,-1)),	// left down
			4 => check_direction(&word_search, (line as i32,column as i32), (0,-1)),	// left
			5 => check_direction(&word_search, (line as i32,column as i32), (-1,-1)),	// left up
			6 => check_direction(&word_search, (line as i32,column as i32), (-1,0)),	// up
			7 => check_direction(&word_search, (line as i32,column as i32), (-1,1)),	// right up
			_ => false 
		} as usize;
	}
	num_xmas
}

fn check_x_mas(word_search: &Vec<Vec<char>>, line: usize, column: usize) -> bool {
	if check_out_of_bounds_x_mas(word_search.len(), word_search[0].len(), (line as i32, column as i32)) {
		return false;
	}
	let mas = match word_search[line-1][column-1] {
		'S' => word_search[line+1][column+1] == 'M',
		'M' => word_search[line+1][column+1] == 'S',
		_ => false
	} && match word_search[line-1][column+1] {
		'S' => word_search[line+1][column-1] == 'M',
		'M' => word_search[line+1][column-1] == 'S',
		_ => false
	};
	mas
}
 
fn solve_part1(input: Vec<Vec<char>>) -> usize {
	let num_lines = input.len();
	let num_columns = input[0].len();
	let mut sum: usize = 0;
	for line in 0..num_lines {
		for letter in 0..num_columns {
			if input[line][letter] == 'X' {
				sum += check_xmas(&input, line, letter);
			}
		}
	}
	sum
} 
 
fn solve_part2(input: Vec<Vec<char>>) -> usize { 
	let num_lines = input.len();
	let num_columns = input[0].len();
	let mut sum: usize = 0;
	for line in 0..num_lines {
		for letter in 0..num_columns {
			if input[line][letter] == 'A' {
				sum += check_x_mas(&input, line, letter) as usize;
			}
		}
	}
	sum
} 
