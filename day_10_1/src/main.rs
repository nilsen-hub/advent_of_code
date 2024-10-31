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
struct location {
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
    //load parsed data into step_counter()
    //get amount of steps to complete circuit, divide by 2 and output
}
fn step_counter(data: &Vec<Vec<char>>) { // -> i32
                                         // assign 0 to mut step counter
                                         // find start Location by pulling your boostraps()
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
fn bootstraps(data: &Vec<Vec<char>>) { // -> Location
                                       // find 'S' in data, store index, probably use
                                       // enumerated for-loop for this, inefficient but yields
                                       // clean index value.
                                       // start north of 'S'and go around the clock looking for valid
                                       // symbols using this table:
                                       // valid_north == 'F', '7', '|'
                                       // valid east == '7'. 'J', '-'
                                       // if north is valid, assume 'S' is '|'
                                       // if east is valid, assume 'S' is '-'
                                       // if neither are valid, assume 'S' is '|'
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
