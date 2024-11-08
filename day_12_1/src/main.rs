use std::{fs::read_to_string, time::Instant};

struct Condition {
    numbers: Vec<i32>,
    symbols: Vec<char>,
}
fn main() {
    let now = Instant::now();
    let path = "./data/day12T";
    let full_data = get_list_from_file(path);
    
    
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn build_condition(line: String){
    // -> Condition
    // build instance of Condition
    // Split string on whitespace
    // convert left side into vector of char
    // split right side on ','
    // convert right side into vector of i32
    // define and return "Condition"
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

mod tests {
    use super::*;
    #[test]
    fn calculates_distance() {
        let time = 7;
        let push_time = 2;
        let result = distance_calculator(time, push_time);
        let expected: usize = 10;
        assert_eq!(result, expected);
    }
}