use std::{
    fs::read_to_string,
};

/*
Truth-table
  
    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    
    .....
    .F-7.
    .|.|.
    .L-J.
    .....

    Pairs:
    (|, J) = west 
    (|, L) = east
    (|, 7) = west
    (|, F) = east
    (-, J) = north
    (-, L) = north
    (-, 7) = south
    (-, F) = south
    (J, |) = north
    (L, |) = north
    (7, |) = south
    (F, |) = south
    (J, -) = west
    (L, -) = east
    (7, -) = west
    (F, -) = east
*/


fn main() {
    // load data from file
    let path = "./data/day10";
    let full_data = get_list_from_file(path);
    // parse file into vec<vec<char>>
    let parsed = parse(full_data);
    //load parsed data into step_counter()
    //get answer to puzzle and print to console
}
fn step_counter(data:&Vec<Vec<char>>){// -> i32
    // find start location and direction
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn parse(data: Vec<String>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::new();
    for string in data{
        let chars:Vec<char> = string.chars().collect();
        output.push(chars);
    }

    output
}
