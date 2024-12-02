pub fn solve(input: String) -> (usize, usize) {
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1, part2) 
} 

fn parse_input(input: String) -> Vec<Vec<usize>> {
	let mut data: Vec<Vec<usize>> = Vec::new();
	for report_line in input.lines() {
		let mut report: Vec<usize> = Vec::new();
		for num in report_line.split(' '){
			report.push(num.parse().unwrap());
		}
		data.push(report);
	}
	data
}

fn sign(number: i32) -> i8 {
	if number < 0 {
		-1
	} else if number > 0 {
		1
	} else { 
		0
	}
}

fn is_safe(report: Vec<usize>) -> bool {
	let mut previous: i32 = 0;
	for element in report.iter().enumerate() {
		if (element.0 != 0) {
			if ((*element.1) as i32).abs_diff(previous) < 1 || ((*element.1) as i32).abs_diff(previous) > 3 {
				return false;
			}
			if sign((*element.1) as i32 - previous) != sign(report[1] as i32 - report[0] as i32) {
				return false;
			}
		}
		previous = *element.1 as i32;
	}
	true
}

fn solve_part1(input: String) -> usize {
	let data: Vec<Vec<usize>> = parse_input(input);
	let mut sum: usize = 0;
	sum += data.iter().filter(|&report| {is_safe(report.clone())}).count();
	sum
} 
 
fn solve_part2(input: String) -> usize { 
	let data = parse_input(input);
	let mut sum: usize = 0;
	let mut dampened: Vec<usize> = Vec::new();
	for unsafe_report in data.iter().filter(|&report| {!is_safe((*report).clone())}) {
		for pos in 0..unsafe_report.len() {
			dampened = unsafe_report.iter().take(pos).chain(unsafe_report.iter().skip(pos+1)).map(|x| *x).collect();
			if is_safe(dampened) {
				sum += 1;
				break;
			}
		}
	}
	sum += data.iter().filter(|&report| {is_safe((*report).clone())}).count();
	sum
} 
