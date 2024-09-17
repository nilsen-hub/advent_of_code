//Advent of code 2023, day 2 part one

use std::fs::read_to_string;

fn main() {

    let mut value_accumulator: u32 = 0;
    let path = "./data/day_2_1";
    let full_data = get_list_from_file(path);
    
    for data in full_data{
        let splits = split_machine(data);
        for el in &splits{
            print!("{} ", el);
        }
        let is_fine = fine(&splits);
        if is_fine == true{
            let number: u32 = splits[1].parse().expect("NaN");
            value_accumulator += number;
            println!{" "};
            println!("is fine, added game {} to accumulator", number);
        } else{
            println!("Is TRASHHH");
        }
    }
    println!("{}", value_accumulator);
}

fn fine(input: &Vec<String>) -> bool{

    let mut count = 0;
    for el in input{
        count += 1;
        let el: u32 = match el.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if el > 12 && count > 1{
            let color: &String = &input[count];
            if color == "red"{
                println!(" ");
                println!("color {}, number {}, counter is at index {}", color, el, count);
                return false
            }
            if color == "green" && el > 13{
                println!(" ");
                println!("color {}, number {}, counter is at index {}", color, el, count);
                return false
            }
            if color == "blue" && el > 14{
                println!(" ");
                println!("color {}, number {}, counter is at index {}", color, el, count);
                return false
            }
        }
        
    }
    true
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
