static FIELD_SIZE: (i32, i32) = (101, 103);

pub fn solve(input: String) -> (usize, usize) {
	let robots: Vec<Robot> = input.lines().map(|line| parse_robot(line)).collect();
	let part1 = solve_part1(robots.clone());
	let part2 = solve_part2(robots);
    (part1, part2)
}

fn parse_robot(line: &str) -> Robot {
	let split = line.split(" ").collect::<Vec<&str>>();
	assert_eq!(split.len(), 2);
	let pos = split[0]
		.split(',')
		.map(|part| part.chars().filter(char::is_ascii_digit).collect::<String>().parse::<i32>().unwrap())
		.collect::<Vec<i32>>();
	let vel = split[1]
		.split(',')
		.map(|part| part.chars().filter(|c| *c != 'v' && *c != '=').collect::<String>().parse::<i32>().unwrap())
		.collect::<Vec<i32>>();
	assert_eq!(pos.len(), 2);
	assert_eq!(vel.len(), 2);
	Robot{pos: (pos[0], pos[1]), vel: (vel[0], vel[1])}
}

#[derive(Clone, Debug)]
pub struct Robot {
	pub pos: (i32, i32),
	pub vel: (i32, i32),
}

impl Robot{
	pub fn step(&mut self){
		self.pos.0 += self.vel.0;
		self.pos.1 += self.vel.1;
		if self.pos.0 < 0 || self.pos.0 >= FIELD_SIZE.0 {
			self.pos.0 = (self.pos.0.abs() - FIELD_SIZE.0).abs();
		}
		if self.pos.1 < 0 || self.pos.1 >= FIELD_SIZE.1 {
			self.pos.1 = (self.pos.1.abs() - FIELD_SIZE.1).abs();
		}
	}
}
 
fn solve_part1(mut robots: Vec<Robot>) -> usize {
	for _ in 0..100 {
		robots.iter_mut().for_each(|r| r.step());
	}
	let first_quadrant = robots.iter().filter(|r| r.pos.0 < FIELD_SIZE.0/2 && r.pos.1 < FIELD_SIZE.1/2).count();
	let second_quadrant = robots.iter().filter(|r| r.pos.0 > FIELD_SIZE.0/2 && r.pos.1 < FIELD_SIZE.1/2).count();
	let third_quadrant = robots.iter().filter(|r| r.pos.0 < FIELD_SIZE.0/2 && r.pos.1 > FIELD_SIZE.1/2).count();
	let fourth_quadrant = robots.iter().filter(|r| r.pos.0 > FIELD_SIZE.0/2 && r.pos.1 > FIELD_SIZE.1/2).count();
	println!("{} {} {} {}", first_quadrant, second_quadrant, third_quadrant, fourth_quadrant);
	first_quadrant * second_quadrant * third_quadrant * fourth_quadrant
} 
 
fn solve_part2(mut robots: Vec<Robot>) -> usize {
	let mut i = 1;
	loop {
		robots.iter_mut().for_each(|r| r.step());
		let mut map: Vec<Vec<char>> = vec![vec!['.'; FIELD_SIZE.0 as usize]; FIELD_SIZE.1 as usize];
		for robot in &robots {
			map[robot.pos.1 as usize][robot.pos.0 as usize] = '*';
		}
		if map.iter().any(|r| r.iter().collect::<String>().contains("************")) {
			break;
		}
		i += 1;
	}
	i
} 
