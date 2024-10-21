use std::{fs::read_to_string, i64, iter::Enumerate};
fn main() {
    let path = "./data/day_6T";
    let full_data = get_list_from_file(path);
}

fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn distance_calculator() {
    let race_limit = 7;
    let to_beat = 9;

    let push_time = 2;
    let distance = race_limit - push_time + (push_time * (race_limit - push_time));
    println!("distance: {}", distance);
}
