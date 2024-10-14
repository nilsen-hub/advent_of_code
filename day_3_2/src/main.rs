// Advent of code 2023 day 3 part 1, kernels and stuff.

use std::fs::read_to_string;

fn main() {
    let mut value_accumulator: u32 = 0;
    let path = "./data/day_3_1";
    let full_data = get_list_from_file(path);
    let row_len = get_row_length(&full_data);
    let flattened = vector_cat(&full_data);
    let boundry = flattened.len();

    for (index, _c) in flattened.iter().enumerate() {
        let gear = gear_finder(index, &flattened, row_len, boundry);
        if gear.2 == true {
            println!("index {} is a gear!", index);
            value_accumulator += gear.0 * gear.1;
        }
    }
    println!("The value acummulator got {}", value_accumulator);
}
fn gear_finder(index: usize, data: &Vec<char>, row_len: usize, boundry: usize) -> (u32, u32, bool) {
    let mut output: (u32, u32, bool) = (0, 0, false);
    if is_gearish(data[index]) != true {
        return output;
    }
    let kernel: Vec<usize> = kernel_generator(index, row_len, boundry);
    let mut gear_number_index: Vec<usize> = vec![0];
    let mut gear_number: u32;
    let mut gear_list: Vec<u32> = Vec::new();

    for el in kernel {
        if data[el].is_numeric() {
            if el <= gear_number_index[gear_number_index.len() - 1] {
                continue;
            }

            gear_number_index = number_cat(el, data, row_len);
            gear_number = make_number(&gear_number_index, &data);
            gear_list.push(gear_number);
        }
    }

    if gear_list.len() == 2 {
        output.0 = gear_list[0];
        output.1 = gear_list[1];
        output.2 = true;
    }

    return output;
}
fn make_number(input: &Vec<usize>, data: &Vec<char>) -> u32 {
    let mut num_string = String::new();
    for el in input {
        num_string.push(data[*el]);
    }
    let output = num_string.parse::<u32>().unwrap();
    output
}
fn kernel_generator(index: usize, row_len: usize, boundry: usize) -> Vec<usize> {
    //Function takes an index from a flattened 2D array, along with row length and
    //length definitions, and returns a vector of the indices immediatly surrounding
    //the analyzed index. The vector reads like text, top left first, bottom right last.

    if is_edge(&index, row_len, boundry) {
        let kernel: Vec<usize> = edge_demystifier(index, row_len, boundry);
        println!("This is an edge case!");
        return kernel;
    } else {
        let kernel: Vec<usize> = vec![
            index - (row_len + 1),
            index - row_len,
            index - (row_len - 1),
            index - 1,
            index + 1,
            index + (row_len - 1),
            index + row_len,
            index + (row_len + 1),
        ];
        return kernel;
    }
}
fn edge_demystifier(index: usize, row_len: usize, boundry: usize) -> Vec<usize> {
    let north_w = index - (row_len + 1);
    let north = index - row_len;
    let north_e = index - (row_len - 1);
    let west = index - 1;
    let east = index + 1;
    let south_w = index + (row_len - 1);
    let south = index + row_len;
    let south_e = index + (row_len + 1);

    if index == 0 {
        let top_left: Vec<usize> = vec![east, south, south_e];
        return top_left;
    }
    if (index as i32) - (row_len as i32) < 0 && index % row_len == row_len - 1 {
        let top_right: Vec<usize> = vec![west, south_w, south];
        return top_right;
    }
    if index % row_len == 0 && index + row_len >= boundry {
        let bottom_left: Vec<usize> = vec![north, north_e, east];
        return bottom_left;
    }
    if index + 1 >= boundry {
        let bottom_right: Vec<usize> = vec![north_w, north, west];
        return bottom_right;
    }
    if (index as i32) - (row_len as i32) < 0 {
        let top: Vec<usize> = vec![west, east, south_w, south, south_e];
        return top;
    }
    if index % row_len == 0 {
        let left: Vec<usize> = vec![north, north_e, east, south, south_e];
        return left;
    }
    if index % row_len == row_len - 1 {
        let right: Vec<usize> = vec![north_w, north, west, south_w, south];
        return right;
    }
    if index + row_len > boundry {
        let bottom: Vec<usize> = vec![north_e, north, north_w, west, east];
        return bottom;
    } else {
        let error: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        println!("ERRORERRORERROR");
        return error;
    }
}
fn number_cat(index: usize, data: &Vec<char>, len: usize) -> Vec<usize> {
    let mut counter: usize = index;
    let mut output: Vec<usize> = Vec::new();

    // This loop checks if the number started before the given index
    // number cat returns a vector of indices, this output must be run through
    // fn make_number to yield a scalar value
    loop {
        if data[counter - 1].is_numeric() && counter != 0 {
            counter -= 1;
            if counter % len == len - 1 || counter % len == 0 {
                break;
            }
        } else {
            break;
        }
    }

    loop {
        if data[counter].is_numeric() {
            output.push(counter);
            if counter % len == len - 1 {
                break;
            }
        } else {
            break;
        }
        counter += 1;
    }
    output
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn get_row_length(input: &Vec<String>) -> usize {
    let mut output = 0;
    for el in input {
        output = el.len();
        break;
    }
    return output;
}
fn vector_cat(input: &Vec<String>) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    for el in input {
        let my_chars: Vec<_> = el.chars().collect();
        for el in my_chars {
            output.push(el);
        }
    }
    return output;
}
fn is_edge(index: &usize, len: usize, boundry: usize) -> bool {
    let edge_list: Vec<bool> = vec![
        (*index as i32) - (len as i32) < 0,
        index % len == 0,
        index % len == len - 1,
        index + len > boundry,
    ];

    for e in edge_list {
        if e {
            return true;
        }
    }

    false
}
fn is_gearish(input: char) -> bool {
    input == '*'
}
