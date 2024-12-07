pub fn solve(input: String) -> (usize, usize) { 
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
}

fn parse_line(line: &str) -> (usize, Vec<usize>) {
	let split = line.split(':').collect::<Vec<&str>>();
	assert_eq!(split.len(), 2);
	let calibration = split[0].parse().unwrap();
	let equation = split[1].trim().split(' ').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
	(calibration, equation)
}

fn check_equation(calibration: usize, equation: &Vec<usize>) -> bool {
	for i in 0..2_i32.pow((equation.len() - 1) as u32) {
		let mut result = equation[0];
		for j in 1..equation.len() {
			match (i >> j-1) & 1 {
				0 => result = result + equation[j],
				1 => result = result * equation[j],
				_ => panic!("something went wrong while calculating the equation")
			}
		}
		if result == calibration {
			return true;
		}
	}
	false
}

fn check_equation_2(calibration: usize, equation: &Vec<usize>) -> bool {
	let mut test_case: Vec<u8> = Vec::new();
	for mut i in 0..3_i32.pow((equation.len() - 1) as u32) {
		for digit in (0..equation.len()-1).rev() {
			test_case.push((i / 3_i32.pow(digit as u32)) as u8);
			i = i % 3_i32.pow(digit as u32);
		}
		let mut result = equation[0];
		for j in 1..equation.len() {
			match test_case[j-1] {
				0 => result = result + equation[j],
				1 => result = result * equation[j],
				2 => result = (result.to_string() + &*equation[j].to_string()).parse::<usize>().unwrap(),
				_ => panic!("something went wrong while calculating the equation")
			}
		}
		if result == calibration {
			return true;
		}
		test_case.clear();
	}
	false
}
 
fn solve_part1(input: String) -> usize { 
	let parsed_input: Vec<(usize, Vec<usize>)> = input.lines().map(|line| parse_line(line)).collect();
	parsed_input
		.iter()
		.filter(|(calibration, equation)| check_equation(*calibration, equation))
		.fold(0, |acc, x| acc + x.0)
} 
 
fn solve_part2(input: String) -> usize { 
	let parsed_input: Vec<(usize, Vec<usize>)> = input.lines().map(|line| parse_line(line)).collect();
	parsed_input
		.iter()
		.filter(|(calibration, equation)| check_equation(*calibration, equation) || check_equation_2(*calibration, equation))
		.fold(0, |acc, x| acc + x.0)
} 
