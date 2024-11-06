use std::{collections::HashSet, fs::read_to_string, ops::Index, time::Instant};

fn main() {
    let now = Instant::now();

    let path = "./data/day11T";
    let full_data = get_list_from_file(path);
    // parse data into Vec<Vec<char>>
    let parsed: Vec<Vec<char>> = parse(full_data);
    // apply expansion algo to parsed data, use the pump()
    let inflated = pump(parsed);
    // gather list of galaxies as indices into Vec<(usize, usize)> from
    // the galactic_index().
    let galaxies = galactic_index(&inflated);
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
    // for every item
    // find absolute difference between item.0 and item.1 in every following index
    // add differnce.item.0 and difference.item.1 add to
    // distance accumulator
    for (index, coords) in galaxies.iter().enumerate(){
    
        let mut idx = boundary;
        while idx != index{
            let x_distance = coords.0.abs_diff(galaxies[idx].0); 
            let y_distance = coords.1.abs_diff(galaxies[idx].1);
            distance_accumulator += (x_distance + y_distance);
            idx -= 1; 
        }
    }

    distance_accumulator

    
    // return distance accumulator
}
fn galactic_index(data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    // returns vector of tuples containing coords to the galaxies

    // instantiate vctor of tuples, tuples represent coord x, coord y
    let mut coords:Vec<(usize, usize)> = Vec::with_capacity(512);
    for (idx_row, row) in data.iter().enumerate(){
        for (idx_col, el) in row.iter().enumerate(){
            if *el == '#'{
                coords.push((idx_col, idx_row));
            }
        }
    }
    coords
}   
fn pump(mut data:Vec<Vec<char>>) -> Vec<Vec<char>> {
    // expands data according to the entropic laws of this universe

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
    
    // sort vectors in decending order
    y_empty.sort();
    x_empty.sort();
    y_empty.reverse();
    x_empty.reverse();

    // instantiate empty dummy row 
    let dummy: Vec<char> = vec!['.'; data[1].len()];

    // expand y-axis
    for index in y_empty{
        data.insert(index,dummy.clone());
    }
    for mut row in &mut data{
        for index in &x_empty{
            row.insert(*index, '.');
        }
    }

    for line in &data{
        println!("{:?}", line);
    }

    data
    // return refreshed vector
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
