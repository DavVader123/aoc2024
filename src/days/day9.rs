pub fn solve(input: String) -> (usize, usize) {
	let file_layout = parse_disk_map(&input);
	let part1 = solve_part1(file_layout.clone());
	let part2 = solve_part2(file_layout);
    (part1, part2) 
}

fn parse_disk_map(disk_map: &String) -> Vec<Vec<i16>> {
	let mut file_layout: Vec<Vec<i16>> = Vec::new();
	for digit in disk_map.chars().enumerate() {
		let test = String::from(digit.1).parse::<usize>().unwrap();
		if test > 0 {
			match digit.0 % 2 {
				0 => file_layout.push(std::iter::repeat((digit.0 / 2) as i16).take(test).collect()),
				1 => file_layout.push(std::iter::repeat(-1).take(test).collect()),
				_ => panic!("this definitely shouldn´t happen!"),
			}
		}
	}
	file_layout
}
 
fn solve_part1(file_layout: Vec<Vec<i16>>) -> usize {
	let mut file_layout: Vec<i16> = file_layout.iter().flatten().copied().collect();
	let (mut i, mut j): (usize,usize) = (0,file_layout.len()-1);
	while i <= j {
		if file_layout[i] == -1 && file_layout[j] >= 0 {
			file_layout.swap(i,j);
		}
		if file_layout[i] >= 0 {
			i += 1;
		}
		if file_layout[j] == -1 {
			file_layout.remove(j);
			j -= 1;
		}
	}
	file_layout.iter().enumerate().fold(0, |acc, (i,file_id)| acc + i * (*file_id as usize))
}
 
fn solve_part2(mut file_layout: Vec<Vec<i16>>) -> usize {
	for h in 1..usize::MAX {
		if h > file_layout.len() {
			break;
		}
		let i = file_layout.len()-h;
		let file_length =file_layout[i].len();
		if file_layout[i][0] >= 0 {
			for j in 0..i {
				let free_length = file_layout[j].len();
				if file_layout[j][0] == -1 && free_length >= file_length {
					let file = file_layout.remove(i);
					file_layout.insert(i, vec![-1; file_length]);
					if free_length == file_length {
						file_layout.remove(j);
					} else {
						file_layout[j].truncate(free_length - file_length);
					}
					file_layout.insert(j,file);
					break;
				}
			}
		}
	}
	file_layout.iter().flatten().copied().enumerate().filter(|(_i, file_id)| *file_id >= 0).fold(0, |acc, (i,file_id)| acc + i * (file_id as usize))
} 
