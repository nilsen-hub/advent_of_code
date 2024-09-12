//Advent of code day 1 part 2 - grab first and last number from a string
//concatenate the two in order and reutrn a two-digit number. Repeat for list of 
//many strings and sum all two digit numbers for answer. 
//Part 2 particulars: some numbers are written as "one" "two" etc. These also count

use std::fs::read_to_string;
use std::io;

fn main() {
    println!("Please enter filepath to list of bungled calibration values");
    
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
        
    let path = path.trim();

    let data = get_list_from_file(path);
    let mut cal_val = Vec::new();
    
    for el in data{
        let val = grab_number(el);
        cal_val.push(val);
    }

    let output = sum_vector(cal_val);
    println!("{output}");
}

fn grab_number(data: String) -> u32 {
    let mut flag = 0;
    let mut num_1 = '0';
    let mut num_2 = '0'; 
   
    for c in data.chars(){
        if c.is_numeric(){
            if flag == 0{
                num_1 = c;
                flag += 1;
            }
            num_2 = c;            
        } 
    }

    let number = format!("{}{}", num_1, num_2);
    
    let number: u32 = match number.parse() {
        Ok(num) => num,
        Err(_)  => todo!(),
    };

    number
}

fn get_list_from_file(path: &str) -> Vec<String>{
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn sum_vector(vector: Vec<u32>) -> u32 {
    let mut sum = 0;
    for el in vector{
        sum = sum + el;
    }

    sum
}