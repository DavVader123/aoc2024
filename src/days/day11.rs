use std::collections::HashMap;
use rayon::prelude::*;

pub fn solve(input: String) -> (usize, usize) {
    let part1 = solve_part1(input.clone());
    let part2 = solve_part2(input.clone());
    (part1, part2)
}

fn blink(stone: usize) -> Vec<(usize, usize)> {
	if stone == 0 {
		return vec!((1,1));
	}
	let num_as_str = stone.to_string();
	if num_as_str.len() % 2 == 0 {
		let split = num_as_str.split_at(num_as_str.len() / 2);
		return vec!((split.0.parse().unwrap(),1), (split.1.parse().unwrap(),1));
	}
	vec!((stone*2024,1))
}

fn blink_n_times(stones: &mut HashMap<usize, usize>, num_blinks: usize) {
	for _ in 0..num_blinks {
		let stones_clone = stones.clone();
		let stones_keys = stones.clone().into_keys();
		stones.clear();
		for stone in stones_keys {
			let new = blink(stone);
			let number = stones_clone[&stone];
			for new_stone in new {
				stones.insert(new_stone.0, stones.get(&new_stone.0).unwrap_or(&0) + new_stone.1 * number);
			}
		}
	}
}

fn solve_part1(input: String) -> usize {
	let stones: Vec<usize> = input.split(' ').map(|x| x.parse().unwrap()).collect();
	let mut stones: HashMap<usize, usize> = stones.iter().map(|x| (*x, 1)).collect();
	blink_n_times(&mut stones, 25);
	stones.values().sum()
}

fn solve_part2(input: String) -> usize {
    let stones: Vec<usize> = input.split(' ').map(|x| x.parse().unwrap()).collect();
	let mut stones: HashMap<usize, usize> = stones.iter().map(|x| (*x, 1)).collect();
	blink_n_times(&mut stones, 75);
	stones.values().sum()
}
