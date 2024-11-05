use std::{fs::read_to_string, time::Instant};

fn main() {
    let now = Instant::now();

    let path = "./data/day11T";
    let full_data = get_list_from_file(path);
    // parse data into Vec<Vec<char>>
    let parsed: Vec<Vec<char>> = parse(full_data);
    // apply expansion algo to parsed data, use entropyfi()
    // gather list of galaxies as indices into Vec<(usize, usize)> from
    // the galactic_index().
    // give list of galaxies to the igmd() (intra galactic measuring device)
    // print results to console

    println!("program runtime: {}", now.elapsed().as_micros());
}
fn igmd(galaxies:&Vec<(usize, usize)>){
    // intragalactic measuring device
    // -> u32
    // assign 0 to distance_accumulator: u32
    // assign galaxy.len() - 1 to boundary
    // start indexed loop over galaxies
        // for every item
            // find absolute difference between item.0 and item.1 in every following index
            // add differnce.item.0 and difference.item.1, substract 1 from sum and add to
            // distance accumulator
    
    // return distance accumulator
}
fn galactic_index(Vec<Vec<char>>){
    // -> Vec<(usize, usize)>
    // loop through data and gather indices of all #-occurences into a vector
    // return that vector
}   
fn entropyfi(data:Vec<Vec<char>>){
    // -> Vec<Vec<char>> 
    // expand X axis
    // expand y axis
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
