
pub fn get_input(day: i32) -> String {
    let path: String = format!("./input/input{}.txt", day);
    let contents = std::fs::read_to_string(path).expect("Something went wrong reading the file {}");
    contents
}