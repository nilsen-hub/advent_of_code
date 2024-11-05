use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();
    // program in this gap
    let path = "./data/day11T";
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
