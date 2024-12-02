pub fn solve(input: String) -> (usize, usize) {
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
} 
 
fn solve_part1(input: String) -> usize {
	for report in input.lines() {
		report.split(' ').for_each();
	}
	0
} 
 
fn solve_part2(input: String) -> usize { 
0 
} 
