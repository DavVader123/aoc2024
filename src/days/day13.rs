use round::*;

pub fn solve(input: String) -> (usize, usize) {
	let claw_machines: Vec<ClawMachine> = parse_input(input);
	let part1 = solve_part1(claw_machines.clone());
	let part2 = solve_part2(claw_machines);
    (part1, part2) 
}

#[derive(Debug, Clone)]
pub struct ClawMachine {
	pub a: (f64, f64),
	pub b: (f64, f64),
	pub goal: (f64, f64),
}

fn parse_input(input: String) -> Vec<ClawMachine> {
	let mut result: Vec<ClawMachine> = Vec::new();
	let lines = input.lines().map(|line| line.to_string()).collect::<Vec<String>>();
	let mut i: usize = 0;
	while i < lines.len() {
		let mut machine = ClawMachine{
			a: (0f64, 0f64),
			b: (0f64, 0f64),
			goal: (0f64, 0f64),
		};
		let values = lines[i]
			.split(",")
			.map(|val| {val.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<f64>().unwrap()})
			.collect::<Vec<f64>>();
		assert_eq!(values.len(), 2);
		machine.a = (values[0], values[1]);
		i += 1;
		let values = lines[i]
			.split(",")
			.map(|val| {val.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<f64>().unwrap()})
			.collect::<Vec<f64>>();
		assert_eq!(values.len(), 2);
		machine.b = (values[0], values[1]);
		i += 1;
		let values = lines[i]
			.split(",")
			.map(|val| {val.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<f64>().unwrap()})
			.collect::<Vec<f64>>();
		assert_eq!(values.len(), 2);
		machine.goal = (values[0], values[1]);
		result.push(machine);
		i += 2;
	}
	result
}

fn win_cost(machine: &ClawMachine) -> usize {
	if machine.a.0/machine.a.1 == machine.b.0/machine.b.1 {
		println!("passiert des überhaupt?");
		if machine.a.0/machine.a.1 == machine.goal.0/machine.goal.1 {
			for i in 0..i32::MAX {
				let temp = ((machine.goal.0 - (i as f64 * machine.a.0)) / machine.b.0);
				if temp.fract() < 0.0001 || temp.fract() > 0.9999 {
					return 3*i as usize + round(temp,0) as usize;
				}
			}
		} else {
			return 0;
		}
	}
	let b = ((machine.goal.1 * machine.a.0) - (machine.goal.0 * machine.a.1)) / 
				  (machine.b.1 * machine.a.0 - machine.b.0 * machine.a.1);
	if b.fract() > 0.0001 && b.fract() < 0.9999 {
		return 0;
	}
	let a1 = (machine.goal.0-(b*machine.b.0))/machine.a.0;
	let a2 = (machine.goal.1-(b*machine.b.1))/machine.a.1;
	if a1.fract() > 0.0001 && a1.fract() < 0.9999 || a2.fract() > 0.0001 && a2.fract() < 0.9999 {
		return 0;
	}
	if  round(a1,1) == round(a2,1) {
		return round(a1,0) as usize * 3 + round(b,0) as usize;
	}
	0
}

fn solve_part1(claw_machines: Vec<ClawMachine>) -> usize {
	let mut sum: usize = 0;
	for machine in claw_machines {
		sum += win_cost(&machine);
	}
	sum
} 
 
fn solve_part2(mut claw_machines: Vec<ClawMachine>) -> usize {
	let mut sum: usize = 0;
	claw_machines.iter_mut().for_each(|machine| {machine.goal.0 += 10000000000000.0; machine.goal.1 += 10000000000000.0} );
	for machine in claw_machines {
		sum += win_cost(&machine);
	}
	sum
} 
