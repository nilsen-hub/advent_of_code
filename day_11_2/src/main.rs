use std::{collections::HashSet, fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();

    let path = "./data/day11";
    let full_data = get_list_from_file(path);
    // parse data into Vec<Vec<char>>
    let parsed: Vec<Vec<char>> = parse(full_data);
    // apply expansion algo to parsed data, use the pump()
    //let inflation_map = 
    let inflation_map =inflation_mapper(&parsed);
    // gather list of galaxies as indices into Vec<(usize, usize)> from
    // the galactic_index().
    let galaxies = galactic_index(&parsed, inflation_map);
    // give list of galaxies to the igmd() (intra galactic measuring device)
    let total_distance = igmd(&galaxies);
    // print results to console
    println!("The total distance between all galaxies is {} steps", total_distance);
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn igmd(galaxies:&Vec<(usize, usize)>) -> usize {
    // intragalactic measuring device

    // assign 0 to distance_accumulator: u32
    let mut distance_accumulator: usize = 0;
    // assign galaxy.len() - 1 to boundary
    let boundary = galaxies.len() - 1;
    // start indexed loop over galaxies
    // find absolute difference between item.0 and item.1 in every following index
    // add differnce.item.0 and difference.item.1 add to
    // distance accumulator
    for (index, coords) in galaxies.iter().enumerate(){
    
        let mut idx = boundary;
        while idx != index{
            let x_distance = coords.0.abs_diff(galaxies[idx].0); 
            let y_distance = coords.1.abs_diff(galaxies[idx].1);
            distance_accumulator += x_distance + y_distance;
            idx -= 1; 
        }
    }

    distance_accumulator
    // return distance accumulator
}
fn galactic_index(data: &Vec<Vec<char>>, inflation_map:(Vec<usize>, Vec<usize>)) -> Vec<(usize, usize)> {
    // returns vector of tuples containing coords to the galaxies

    // instantiate vctor of tuples, tuples represent coord x, coord y
    let mut coords:Vec<(usize, usize)> = Vec::with_capacity(512);
    for (idx_y, row) in data.iter().enumerate(){
        for (idx_x, el) in row.iter().enumerate(){
            if *el == '#'{
                let offset = inflation_calculator((idx_x, idx_y), &inflation_map);
                coords.push((idx_x + offset.0, idx_y + offset.1));
            }
        }
    }
    coords
}
fn inflation_calculator(values:(usize, usize), inflation_map:&(Vec<usize>, Vec<usize>)) -> (usize, usize) {
    // takes coordinates and calculates offsets based on inflation factor
    let inflation_factor: usize = 1000000;
    let mut x_offset: usize = 0;
    let mut y_offset: usize = 0;

    for el in &inflation_map.0{
        if values.0 > *el {
            x_offset += inflation_factor - 1;
        }
    }
    for el in &inflation_map.1{
        if values.1 > *el {
            y_offset += inflation_factor - 1;
        }
    }

    (x_offset, y_offset)
}   
fn inflation_mapper(data:&Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) { 
    // finds the inflation points of the universe

    // make a reference hash set containing all digits from 0 to data[0].len() - 1
    let mut count: usize = data[0].len() - 1;
    let mut reference_set: HashSet<usize> = HashSet::with_capacity(256);
    loop {
        reference_set.insert(count);
        if count == 0{
            break;
        }
        count -= 1;
    }
    
    // instantiate  hash sets x_occupied, y_occupied 
    let mut y_occupied: HashSet<usize> = HashSet::with_capacity(64);
    let mut x_occupied: HashSet<usize> = HashSet::with_capacity(64);
    
    // analyze data for empty columns and rows
    // Use a nested, indexed for-loop, when '#' is encountered add indices to
    // x-y_occupied
    for (idx_outer, row) in data.iter().enumerate(){
        let mut empty_vec_flag:bool = true;
        for (idx_inner, col) in row.iter().enumerate(){
            if *col == '#'{
                x_occupied.insert(idx_inner);
                empty_vec_flag = false;
            }
        }
        if empty_vec_flag == true{
            y_occupied.insert(idx_outer);
        }
    }
    // make vectors x_empty and y_empty and fill with difference between
    // x-y_occupied and reference set to get empty rows and columns
    let mut y_empty:Vec<usize> = Vec::with_capacity(64);
    let mut x_empty:Vec<usize> = Vec::with_capacity(64);

    for val in y_occupied.drain(){
        y_empty.push(val);
    }
    for val in reference_set.difference(&x_occupied){
        x_empty.push(*val);
    }
    
    // sort vectors in ascending order
    y_empty.sort();
    x_empty.sort();

    (x_empty, y_empty)
}
fn parse(data: Vec<String>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::with_capacity(500);
    for string in data {
        let chars: Vec<char> = string.chars().collect();
        output.push(chars);
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
