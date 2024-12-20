use std::{fs::read_to_string, time::Instant};

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
    ('7','L') = east/north
    ('L','7') = west/south
    ('F','J') = north/west
    ('J','F') = east/south
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
    ('7', 'F') = south
    ('F', '7') = south
    ('7', 'J') = west
    ('J', '7') = west
    ('J', 'L') = north
    ('L', 'J') = north
    ('L', 'F') = east
    ('F', 'L') = east

*/

#[derive(Debug)]
struct Location {
    coord: (usize, usize),
    current: char,
    last: char,
    direction: char,
}

fn main() {
    let now = Instant::now();
    // load data from file
    let path = "./data/day10";
    let full_data = get_list_from_file(path);
    // parse file into vec<vec<char>>
    let parsed = parse(full_data);
    let solution = step_recorder(&parsed);
    //load parsed data into step_counter()
    //get amount of steps to complete circuit, divide by 2 and output
    println!(
        "The halfway point of the pipe is {} steps from the origin",
        solution / 2
    );
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn step_recorder(data: &Vec<Vec<char>>) -> i32 {
    // find start Location by pulling your boostraps()
    let mut location = bootstraps(data);
    // initialize step counter with 1 to account for first step in bootstrap
    let mut step_counter = 1;
    // start walk-loop.
    loop {
        // get the next location from pathfinder()
        location = pathfinder(&location, &data);
        step_counter += 1;
        // if location.current == 'S', break loop
        if location.current == 'S' {
            break;
        }
    }

    // return step counter
    step_counter
}
fn pathfinder(location: &Location, data: &Vec<Vec<char>>) -> Location {
    // assign (location.last, location.current) path tuple
    let path: (char, char) = (location.last, location.current);
    let mut coord: (usize, usize) = location.coord;
    let mut direction: char = '0';
    // match tuple to truth table, increment / decrement coords to match
    // direction:
    // north = decrement coord.0
    // south = increment coord.0
    // east = increment coord.1
    // west = decrement coord.1
    match path {
        ('|', 'J') | ('|', '7') | ('J', '-') | ('7', '-') | ('7', 'J') | ('J', '7') => {
            coord.1 -= 1;
            direction = 'W';
        }
        ('|', 'L') | ('|', 'F') | ('L', '-') | ('F', '-') | ('L', 'F') | ('F', 'L') => {
            coord.1 += 1;
            direction = 'E';
        }
        ('-', 'J') | ('-', 'L') | ('J', '|') | ('L', '|') | ('J', 'L') | ('L', 'J') => {
            coord.0 -= 1;
            direction = 'N';
        }
        ('7', '|') | ('F', '|') | ('-', '7') | ('-', 'F') | ('7', 'F') | ('F', '7') => {
            coord.0 += 1;
            direction = 'S';
        }

        // deal with edge cases down here:
        ('|', '|') => {
            if location.direction == 'N' {
                coord.0 -= 1;
                direction = 'N';
            } else {
                coord.0 += 1;
                direction = 'S';
            }
        }
        ('-', '-') => {
            if location.direction == 'W' {
                coord.1 -= 1;
                direction = 'W';
            } else {
                coord.1 += 1;
                direction = 'E';
            }
        }
        ('7', 'L') => {
            if location.direction == 'W' {
                coord.0 -= 1;
                direction = 'N';
            } else {
                coord.1 += 1;
                direction = 'E';
            }
        }
        ('L', '7') => {
            if location.direction == 'N' {
                coord.1 -= 1;
                direction = 'W';
            } else {
                coord.0 += 1;
                direction = 'S';
            }
        }
        ('F', 'J') => {
            if location.direction == 'S' {
                coord.1 -= 1;
                direction = 'W';
            } else {
                coord.0 -= 1;
                direction = 'N';
            }
        }
        ('J', 'F') => {
            if location.direction == 'W' {
                coord.0 += 1;
                direction = 'S';
            } else {
                coord.1 += 1;
                direction = 'E';
            }
        }
        _ => println!("BIG MISTAKE"),
    }
    // assemble new location descriptor "output"
    // coord = location.coord
    // current = data[coord.0][coord.1]
    // last = location.current
    Location {
        coord,
        current: data[coord.0][coord.1],
        last: location.current,
        direction,
    }
    // return output
}
fn bootstraps(data: &Vec<Vec<char>>) -> Location {
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
    // symbols.
    // initalize location variables.
    let mut current: char = '0';
    let mut last: char = '0';
    let mut coord: (usize, usize) = (0, 0);
    let mut direction: char = '0';

    let valid_north: [char; 3] = ['F', '7', '|'];
    let valid_east: [char; 3] = ['7', 'J', '-'];
    let valid_south: [char; 3] = ['L', 'J', '|'];

    for (idx, loc) in checks.iter().enumerate() {
        let symbol: char = data[loc.0][loc.1];
        match idx {
            0 if valid_north.contains(&symbol) => {
                    coord = loc.clone();
                    last = '|';
                    current = symbol.clone();
                    direction = 'N';
                    break;
            }
            1 if valid_east.contains(&symbol) => {
                    coord = loc.clone();
                    last = '-';
                    current = symbol.clone();
                    direction = 'E';
                    break;
            }
            2 if valid_south.contains(&symbol) => {
                    coord = loc.clone();
                    last = '|';
                    current = symbol.clone();
                    direction = 'S';
                    break;
            }
            _ => {
                println!("This shouldnt be possible");
            }
        }
    }

    Location {
        coord,
        current,
        last,
        direction,
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
    let mut output: Vec<Vec<char>> = Vec::with_capacity(500);
    for string in data {
        let chars: Vec<char> = string.chars().collect();
        output.push(chars);
    }

    output
}
