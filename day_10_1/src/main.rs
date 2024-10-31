use std::fs::read_to_string;

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
    ('|', 'J') = west
    ('|', 'L') = east
    ('|', '7') = west
    ('|', 'F') = east
    ('-', 'J') = north
    ('-', 'L') = north
    ('-', '7') = south
    ('-', 'F') = south
    ('J', '|') = north
    ('L', '|') = north
    ('7', '|') = south
    ('F', '|') = south
    ('J', '-') = west
    ('L', '-') = east
    ('7', '-') = west
    ('F', '-') = east
*/

#[derive(Debug)]
struct Location {
    Coord: (usize, usize),
    Current: char,
    Last: char,
}

fn main() {
    // load data from file
    let path = "./data/day10";
    let full_data = get_list_from_file(path);
    // parse file into vec<vec<char>>
    let parsed = parse(full_data);
    step_counter(&parsed);
    //load parsed data into step_counter()
    //get amount of steps to complete circuit, divide by 2 and output
}
fn step_counter(data: &Vec<Vec<char>>) {
    // -> i32
    let mut step_counter = 0; // assign 0 to mut step counter
    let location = bootstraps(data); // find start Location by pulling your boostraps()
                                     // start walk-loop.
                                     // increment step counter
                                     // get the next location from pathfinder()
                                     // if location.current == 'S', break loop
                                     // return step counter
}
fn pathfinder(mut location: &Location, data: &Vec<Vec<char>>) { // -> Location
                                                                // assign (location.last, location.current) path tuple
                                                                // match tuple to truth table, increment / decrement coords to match
                                                                // direction:
                                                                // north = decrement location.coord.0
                                                                // south = increment location.coord.0
                                                                // east = increment location.coord.1
                                                                // west = decrement location.coord.1
                                                                // assemble new location descriptor "output"
                                                                // coord = location.coord
                                                                // current = data[coord.0][coord.1]
                                                                // last = location.current
                                                                // return output
}
fn bootstraps(data: &Vec<Vec<char>>) {
    // -> Location
    // find 'S' in data, store index, probably use
    // enumerated for-loop for this, inefficient but yields
    // clean index value.
    let mut index: (usize, usize) = (0, 0);
    'get_index: for (index_row, el1) in data.iter().enumerate() {
        for (index_col, el2) in el1.iter().enumerate() {
            if *el2 == 'S' {
                index = (index_row, index_col);
                break 'get_index;
            }
        }
    }
    // define north, east and south, relative to S
    let checks: [(usize, usize); 3] = [
        (index.0 - 1, index.1), // north
        (index.0, index.1 + 1), // east
        (index.0 + 1, index.1), // south
    ];
    // start north of 'S'and go around the clock looking for valid
    // symbols using this table:
    // valid_north == 'F', '7', '|'
    // valid east == '7'. 'J', '-'
    // if north is valid, assume 'S' is '|'
    // if east is valid, assume 'S' is '-'
    // if neither are valid, assume 'S' is '|'
    let mut current: char = '0';
    let mut last: char = '0';
    let mut coord: (usize, usize) = (0, 0);

    let north: [char; 3] =['F', '7', '|']
    let east: [char; 3] = ['7','J','-']

    for (idx, loc) in checks.iter().enumerate() {
        let symbol: char = data[loc.0][loc.1];
        match idx {
            0 => {
                if north.contains(symbol){
                    // valid north
                    coord = loc.clone();
                    last = '|';
                    current = symbol.clone();
                    break;
                }
            }
            1 => {
                if east.contains(symbol){
                    // valid east
                    coord = loc.clone();
                    last = '-';
                    current = symbol.clone();
                }
            }
            _ => { coord = loc.clone();
                    last = '|';
                    current = symbol.clone();

            } 
        }
    }

    // assemble location{} descriptor and return
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
    for string in data {
        let chars: Vec<char> = string.chars().collect();
        output.push(chars);
    }

    output
}
