use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    let path = "./data/day12T";
    let full_data = get_list_from_file(path);
    
    
    println!("program runtime: {}", now.elapsed().as_micros());
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