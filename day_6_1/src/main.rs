use std::{fs::read_to_string, i64, iter::{zip, Enumerate}};

fn main() {
    let path = "./data/day_6";
    let full_data = get_list_from_file(path);
    let formatted_data = parse_data(&full_data);
    let answer = day_6_1_solver(formatted_data);
    println!("The answer is: {answer}");
}   

fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn distance_calculator(time: u32, push_time: u32) -> u32 {
    let distance = push_time * (time - push_time);
    distance
}
fn parse_data(data: &Vec<String>) -> Vec<(u32, u32)> {
    /*Parses input data into a vector of tuples. Each tuple represents:
    (time, distance) for the races we are considering */
    let mut numbers: Vec<Vec<u32>> = Vec::with_capacity(2);
    let mut output: Vec<(u32, u32)> = Vec::with_capacity(10);
    for el in data {
        let split_data = el.split_once(":").unwrap();
        let nums:Result<Vec<u32>, _> = split_data.1.split(" ").filter(|&s| !s.is_empty()).map(|x| x.parse()).collect();
        numbers.push(nums.expect("Theres a number here."));
    }
    for (index, el) in numbers[0].iter().enumerate(){
        let tuple = (*el,numbers[1][index]);
        output.push(tuple);
    }
    
    output
}
fn day_6_1_solver(data:Vec<(u32, u32)>) -> u32{
    let mut op_list:Vec<u32> = Vec::with_capacity(5);
    for el in data{
        let distance_to_beat = el.1;
        let mut count = 0;
        let mut wins: u32 = 0;
        while count <= el.0{
            if distance_calculator(el.0, count) > el.1{
                wins += 1;
            }
            count += 1;
        }
        op_list.push(wins);
    }

    vector_multiplier(op_list)
}
fn vector_multiplier(vector: Vec<u32>) -> u32 {
    let mut result: u32 = 1;
    for number in vector{
        result = result*number
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
        let expected: u32 = 10;
        assert_eq!(result, expected);
    }
}


