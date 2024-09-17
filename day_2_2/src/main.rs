//Advent of code 2023, day 2 part one

use std::fs::read_to_string;

fn main() {

    let mut value_accumulator: u32 = 0;
    let path = "./data/day_2_1";
    let full_data = get_list_from_file(path);
    
    for data in full_data{
        let splits = split_machine(data);
        let power_of_game = minimal_game(&splits);
        value_accumulator += power_of_game;
    }
    println!("{}", value_accumulator);
}

fn get_list_from_file(path: &str) -> Vec<String>{
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn split_machine(input: String) -> Vec<String>{
    let work = input.clone().replace(" ",",");
    let work = work.replace(":",",");
    let work = work.replace(";",",");
    let work = work.replace(",,",",");
    let output: Vec<String> = work.split(",").map(String::from).collect();
    output
}

fn minimal_game(input: &Vec<String>) -> u32{
    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;

    let mut color = "white";

    let mut count = 0;
    for el in input{
        count += 1;
        let el: u32 = match el.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if count > 2{
            color = &input[count];

            if color == "red" && el > red_max{
                red_max = el;
            }
            if color == "green" && el > green_max{
                green_max = el;
            }
            if color == "blue" && el > blue_max{
                blue_max = el;
            }
        } 
    }
    return red_max * green_max * blue_max
}