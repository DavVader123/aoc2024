pub fn solve(input: String) -> (usize, usize) { 
	parse_input(input.clone());
	let part1 = solve_part1(input.clone()); 
	let part2 = solve_part2(input.clone()); 
    (part1 as usize, part2 as usize) 
} 

fn parse_input(input: String) -> (Vec<(u16,u16)>, Vec<Vec<u16>>) {
	let rules: Vec<(u16,u16)> = input
		.lines()
		.filter(|line| line.contains('|'))
		.map(|rule| {rule.split('|').map(|num| {num.parse::<u16>().unwrap()}).collect::<Vec<u16>>()})
		.map(|rule| {(rule[0], rule[1])})
		.collect();
	
	let updates: Vec<Vec<u16>> = input
		.lines()
		.filter(|line| line.contains(','))
		.map(|update| {update.split(',').map(|num| {num.parse::<u16>().unwrap()}).collect::<Vec<u16>>()})
		.collect();
	(rules, updates)
}

fn check_update(update: &Vec<u16>, rules: &Vec<(u16,u16)>) -> bool {
	for rule in rules {
		if update.contains(&rule.0) && update.contains(&rule.1) {
			if update.iter().position(|&x| x == rule.0).unwrap() > update.iter().position(|&x| x == rule.1).unwrap() {
				return false;
			}
		}
	}
	true
}

fn order_update(update: &Vec<u16>, rules: &Vec<(u16,u16)>) -> Vec<u16> {
	let mut ordered_update = update.clone();
	while !check_update(&ordered_update, rules) {
		for rule in rules {
			if ordered_update.contains(&rule.0) && ordered_update.contains(&rule.1) {
				let pos1 = ordered_update.iter().position(|&x| x == rule.0).unwrap();
				let pos2 = ordered_update.iter().position(|&x| x == rule.1).unwrap();
				if pos1 > pos2 {
					let elem = ordered_update.remove(pos1);
					ordered_update.insert(pos2, elem);
				}
			}
		}
	}
	assert_eq!(ordered_update.len(), update.len());
	ordered_update
}
 
fn solve_part1(input: String) -> u16 { 
	let(rules, updates): (Vec<(u16,u16)>, Vec<Vec<u16>>) = parse_input(input);
	updates
		.iter()
		.filter(|&update| check_update(update, &rules))
		.map(|update| update[update.len() / 2])
		.sum()
} 
 
fn solve_part2(input: String) -> u16 { 
	let (rules, updates): (Vec<(u16,u16)>, Vec<Vec<u16>>) = parse_input(input);
	updates
		.iter()
		.filter(|&update| !check_update(update, &rules))
		.map(|update| order_update(update, &rules))
		.map(|update| update[update.len() / 2])
		.sum()
} 
