pub fn solve(input: String) -> (usize, usize) { 
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone());
	println!("\n=== Day {:02} ===", 17);
	println!("  · Part 1: {}", part1);
	println!("  · Part 2: {}", part2);
    (0,0) 
}

fn combo_op(op: usize, reg_a: usize, reg_b: usize, reg_c: usize) -> usize {
	if op <= 3 {
		op
	} else if op == 4 {
		reg_a
	} else if op == 5 {
		reg_b
	} else if op == 6 {
		reg_c
	} else {
		panic!("invalid combo op: {}", op);
	}
	
}

fn run_program(mut reg_a: usize, mut reg_b: usize, mut reg_c: usize, program: Vec<usize>) -> Vec<String> {
	let mut instruction_pointer: usize = 0;
	let mut output: Vec<String> = Vec::new();
	loop {
		let mut jump = true;
		match program[instruction_pointer] {
			0 => reg_a = reg_a / (1 << combo_op(program[instruction_pointer + 1], reg_a, reg_b, reg_c)),
			1 => reg_b = reg_b ^ program[instruction_pointer + 1],
			2 => reg_b = combo_op(program[instruction_pointer + 1], reg_a, reg_b, reg_c) % 8,
			3 => if reg_a > 0 && program[instruction_pointer + 1] < program.len() {
				instruction_pointer = program[instruction_pointer + 1];
				jump = false;
			},
			4 => reg_b = reg_b ^ reg_c,
			5 => output.push((combo_op(program[instruction_pointer + 1], reg_a, reg_b, reg_c) % 8).to_string()),
			6 => reg_b = reg_a / (1 << combo_op(program[instruction_pointer + 1], reg_a, reg_b, reg_c)),
			7 => reg_c = reg_a / (1 << combo_op(program[instruction_pointer + 1], reg_a, reg_b, reg_c)),
			_ => panic!("instruction not in instruction set"),
		}
		if jump {
			instruction_pointer += 2;
			if instruction_pointer >= program.len() {
				break;
			}
		}
	}
	output
}
 
fn solve_part1(input: String) -> String {
	let mut input = input.lines();
	let reg_a: usize = input.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
	let reg_b: usize = input.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
	let reg_c: usize = input.next().unwrap().chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
	input.next();
	let program: Vec<usize> = input.next().unwrap().chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as usize).collect();
	let output = run_program(reg_a, reg_b, reg_c, program);
	output.iter().fold(String::new(), |acc, s| acc + s + ",").strip_suffix(",").unwrap().to_string()
}

fn solve_part2(input: String) -> usize {
	let mut reg_a: usize = 0;
	let reg_b: usize = 0;
	let reg_c: usize = 0;
	let program: Vec<usize> = input.lines().nth(4).unwrap().chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as usize).collect();
	let mut i = 1;
	for val in program.iter().rev() {
		loop {
			reg_a = i;
			let output = run_program(reg_a, 0, 0, program.clone());
			println!("{:?}", output);
			i += 1;
			if output[0] == val {
				break;
			}
		}
		i *= 8;
		break;
	}
	0
} 
