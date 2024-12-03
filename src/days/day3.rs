pub fn solve(input: String) -> (usize, usize) {
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1 as usize, part2 as usize)
}

fn parse_input(input: String) -> Vec<(i32, i32)> {
	let mut muls: Vec<(i32, i32)> = Vec::new();
	let mut buf: String = String::new();
	let mut num_buf: String = String::new();
	let (mut num1, mut num2): (i32, i32) = (0,0);
	for c in input.chars() {
		match c {
			'm' => if buf.is_empty() {buf.push('m') } else {buf.clear(); num_buf.clear()},
			'u' => if buf == "m" {buf.push('u') } else {buf.clear(); num_buf.clear()},
			'l' => if buf == "mu" {buf.push('l') } else {buf.clear(); num_buf.clear()},
			'0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => if buf == "mul(" || buf == "mul(," {num_buf.push(c)} else {buf.clear(); num_buf.clear()},
			',' => if buf == "mul(" {buf.push(','); num1 = num_buf.parse::<i32>().unwrap(); num_buf.clear()} else {buf.clear(); num_buf.clear()},
			'd' => if buf.is_empty() {buf.push('d') } else {buf.clear(); num_buf.clear()},
			'o' => if buf == "d" {buf.push('o')} else {buf.clear(); num_buf.clear()},
			'n' => if buf == "do" {buf.push('n')} else {buf.clear(); num_buf.clear()},
			'\'' => if buf == "don" {buf.push('\'')} else {buf.clear(); num_buf.clear()},
			't' => if buf == "don\'" {buf.push('t')} else {buf.clear(); num_buf.clear()},
			'(' => if buf == "mul" || buf == "do" || buf == "don\'t" {buf.push('(')} else {buf.clear(); num_buf.clear()},
			')' => if buf == "mul(," {num2 = num_buf.parse::<i32>().unwrap(); muls.push((num1, num2)); buf.clear(); num_buf.clear()} 
					else if buf == "do(" {muls.push((-1,-1)); buf.clear(); num_buf.clear()}
					else if buf == "don\'t(" {muls.push((-2,-2)); buf.clear(); num_buf.clear()}
					else {buf.clear(); num_buf.clear()},
			_ => {buf.clear(); num_buf.clear()}
		}
	}
	muls
}
 
fn solve_part1(input: String) -> i32 {
	let muls = parse_input(input);
	muls.iter().map(|(a,b)| a*b).sum()
} 
 
fn solve_part2(input: String) -> i32 { 
	let muls = parse_input(input);
	let mut enabled: bool = true;
	let mut sum: i32 = 0;
	for mul in muls {
		if mul == (-1,-1) {
			enabled = true;
			continue;
		}
		if mul == (-2,-2) {
			enabled = false;
			continue;
		}
		if enabled {
			sum += mul.0 * mul.1;
		}
		
	}
	sum
} 
