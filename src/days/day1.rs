 
pub fn solve(input: String) -> (usize,usize) {
    let part1 = solve_part1(input.clone());
    let part2 = solve_part2(input.clone());
    (part1,part2)
}

fn get_two_lists(input: String) -> (Vec<usize>, Vec<usize>) {
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();
    input.lines().for_each(|line| {
        let (left, right) = line.split_at(line.len()/2);
        left_list.push(left.trim().parse::<usize>().unwrap());
        right_list.push(right.trim().parse::<usize>().unwrap());

    });
    (left_list, right_list)
}
fn solve_part1(input: String) -> usize {
    let (mut left_list, mut right_list) = get_two_lists(input);
    left_list.sort();
    right_list.sort();
    assert_eq!(left_list.len(), right_list.len());
    let mut total_distance = 0;
    for i in 0..left_list.len() {
        total_distance += left_list[i].abs_diff(right_list[i]);
    }
    total_distance
}

fn solve_part2(input: String) -> usize {
    let (left_list, right_list) = get_two_lists(input);
    let mut similarity_score = 0;
    for left_id in left_list.iter() {
        let score = left_id * right_list.iter().filter(|&right_id| {*right_id == *left_id}).count();
        similarity_score += score;
    }
    similarity_score
}


