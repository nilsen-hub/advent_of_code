use std::{
    fs::read_to_string,
    i64,
    iter::{zip, Enumerate},
};

fn main() {
    let path = "./data/day_6";
    let full_data = get_list_from_file(path);
    let formatted_data = parse_data(&full_data);
    println!("{}", day_6_2_solver(formatted_data));
}

fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn distance_calculator(time: usize, push_time: usize) -> usize {
    let distance = push_time * (time - push_time);
    distance
}
fn parse_data(data: &Vec<String>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::with_capacity(2);
    for el in data {
        let split_data = el.split_once(":").unwrap();
        let nums: Vec<&str> = split_data.1.split(" ").filter(|&s| !s.is_empty()).collect();
        let nums_cat = nums.concat();
        let nums_as_int: usize = nums_cat.parse().unwrap();
        output.push(nums_as_int);
    }
    println!("{:?}", output);
    output
}
fn day_6_2_solver(data: Vec<usize>) -> usize {
    let distance_to_beat = data[1];
    let mut count = 0;
    let mut wins: usize = 0;
    while count <= data[0] {
        if distance_calculator(data[0], count) > data[1] {
            wins += 1;
        }
        count += 1;
    }
    wins
}
fn vector_multiplier(vector: Vec<usize>) -> usize {
    let mut result: usize = 1;
    for number in vector {
        result = result * number
    }

    result
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
