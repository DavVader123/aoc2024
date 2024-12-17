use priority_queue::DoublePriorityQueue;

pub fn solve(input: String) -> (usize, usize) {
	let parsed: Vec<Vec<char>> = input.trim().lines().map(|line| line.chars().collect()).collect();
	let part1 = solve_part1(parsed.clone());
	let part2 = solve_part2(parsed, part1);
    (part1, part2) 
} 

#[derive(Hash, Copy, Clone, Debug)]
struct Node {
	pub pos: (usize, usize),
	direction: char,
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.pos == other.pos && self.direction == other.direction
	}
}
impl Eq for Node {}

fn solve_part1(mut maze: Vec<Vec<char>>) -> usize {
	let mut explored: DoublePriorityQueue<Node, usize> = DoublePriorityQueue::new();
	let mut frontier: DoublePriorityQueue<Node, usize> = DoublePriorityQueue::new();
	frontier.push(Node { pos: (maze.len()-2, 1), direction: '>' }, 0);
	loop {
		if frontier.is_empty() {
			for node in explored {
				maze[node.0.pos.0][node.0.pos.1] = node.0.direction;
			}
			for line in maze {
				println!("{}", line.iter().collect::<String>());
			}
			panic!("Could not find solution");
		}
		let current = frontier.pop_min().unwrap();
		if maze[current.0.pos.0][current.0.pos.1] == 'E' {
			return current.1;
		}
		explored.push(current.0.clone(), current.1);
		if current.0.pos.0 < maze.len()-1 && maze[current.0.pos.0+1][current.0.pos.1] != '#' {
			let new_node: Node;
			let new_cost: usize;
			if current.0.direction == 'v' {
				new_node = Node{pos: (current.0.pos.0+1, current.0.pos.1), direction: 'v'};
				new_cost = current.1 + 1;
			} else {
				new_node = Node{pos: (current.0.pos.0+1, current.0.pos.1), direction: 'v'};
				new_cost = current.1 + 1001;
			};
			if !explored.iter().any(|node| *node.0 == new_node) && !frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push(new_node.clone(), new_cost);
			} else if frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push_decrease(new_node, new_cost);
			}
		}
		if current.0.pos.0 > 0 && maze[current.0.pos.0-1][current.0.pos.1] != '#' {
			let new_node: Node;
			let new_cost: usize;
			if current.0.direction == '^' {
				new_node = Node{pos: (current.0.pos.0-1, current.0.pos.1), direction: '^'};
				new_cost = current.1 + 1;
			} else {
				new_node = Node{pos: (current.0.pos.0-1, current.0.pos.1), direction: '^'};
				new_cost = current.1 + 1001;
			};
			if !explored.iter().any(|node| *node.0 == new_node) && !frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push(new_node.clone(), new_cost);
			} else if frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push_decrease(new_node, new_cost);
			}
		}
		if current.0.pos.1 < maze[0].len()-1 && maze[current.0.pos.0][current.0.pos.1+1] != '#' {
			let new_node: Node;
			let new_cost: usize;
			if current.0.direction == '>' {
				new_node = Node{pos: (current.0.pos.0, current.0.pos.1+1), direction: '>'};
				new_cost = current.1 + 1;
			} else {
				new_node = Node{pos: (current.0.pos.0, current.0.pos.1+1), direction: '>'};
				new_cost = current.1 + 1001;
			};
			if !explored.iter().any(|node| *node.0 == new_node) && !frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push(new_node.clone(), new_cost);
			} else if frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push_decrease(new_node, new_cost);
			}
		}
		if current.0.pos.1 > 0 && maze[current.0.pos.0][current.0.pos.1-1] != '#' {
			let new_node: Node;
			let new_cost: usize;
			if current.0.direction == '<' {
				new_node = Node{pos: (current.0.pos.0, current.0.pos.1-1), direction: '<'};
				new_cost = current.1 + 1;
			} else {
				new_node = Node{pos: (current.0.pos.0, current.0.pos.1-1), direction: '<'};
				new_cost = current.1 + 1001;
			};
			if !explored.iter().any(|node| *node.0 == new_node) && !frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push(new_node.clone(), new_cost);
			} else if frontier.iter().any(|node| *node.0 == new_node) {
				frontier.push_decrease(new_node, new_cost);
			}
		}
	}
}

fn solve_part2(maze: Vec<Vec<char>>, best: usize) -> usize {
	
	0
} 
