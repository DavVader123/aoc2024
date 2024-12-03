use aoc_parse::Parser;

pub fn solve(input: String) -> (usize, usize) {
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
}

fn parse_input(input: String) -> Vec<String> {
	let mut muls: Vec<String> = Vec::new();
	let mut buf: String = String::new();
	for c in input.chars() {
		if c == 'm' || c == 'u' || c == 'l' || c == '(' || c == ',' || c == ')' || c.is_digit(10) {
			buf.push(c.clone());
		}
	}
	muls
}
 
fn solve_part1(input: String) -> usize { 
	0
} 
 
fn solve_part2(input: String) -> usize { 
	0
} 
