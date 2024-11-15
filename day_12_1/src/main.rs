use std::{fs::read_to_string, iter::Map, time::Instant};

#[derive(Debug, Clone)]
struct SpringGroup {
    id: usize,
    size: usize,
    start_index: usize,
}
#[derive(Debug, Clone)]
struct Maps {
    springs: Vec<char>,
    groups: Vec<usize>,
}

#[derive(Debug, Clone)]
struct ConditionMap {
    maps: Maps,
    first_arrangement: Vec<char>,
    spring_groups: Vec<SpringGroup>,
}

fn main() {
    let now = Instant::now();
    let path = "./data/day12";
    let full_data = get_list_from_file(path);
    let mut value_accumulator: usize = 0;
    for line in full_data {
        //let total_arrangements_in_maps =
        arrangement_coordinator(line);
        //value_accumulator += total_arrangements_in_maps;
    }
    println!(
        "Theres a total of {} possible arrangements in the data provided",
        value_accumulator
    );
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn arrangement_coordinator(line: String) {
    // -> usize
    // parse string line into Vec<char> and Vec<usize>
    // and make maps struct
    let (springs, groups) = parse_line(line);
    let maps = Maps { springs, groups };
    build_cm(maps);
    // DBug | println!("springs: {:?} groups: {:?}", map_springs, map_groups);
    // Get ConditionMap with build_cm()
    // Analyze ConditionMap to determine numbber of unique arrangements
    // return number of unique arrangements
}
fn build_cm(maps: Maps) {
    // -> ConditionMap
    // Get ConditionMap.spring_groups: Vec<SpringGroup>
    build_spr_groups(maps);
    // Get left leaning first valid arrangement
    // Fill and return struct
}
fn build_spr_groups(maps: Maps) {
    // BUG UNCOVERED
    // NEED TO COVER THIS CASE: ??.??????.?#????? 1,4
    // set up definitions and workspace
    // clone map_springs to have a workplace, and a clean reference
    let map_springs = maps.springs;
    let map_groups = maps.groups;
    let mut working_vector = map_springs.clone();
    // we're gonna be indexing around here, so need this to avoid panics
    let bounds = map_springs.len();
    let valid_symbols: [char; 2] = ['?', '#'];
    let mut groups: Vec<SpringGroup> = Vec::with_capacity(8);

    // the goal of this function is to produce a left leaning valid
    // first arrangement of the spring groups

    // Placement rules:
    // '#' MUST be filled
    // '?' can be filled
    // no placement on '.' characters
    // no two groups on adjacent indices

    // to enforce these rules, were going to assume all inputs are valid
    // and then pass them through a series of "gates" to mark as false
    // if needed, as we move through the data. This is probably a naive
    // approach but for now its the best I've come up with. Theres a lot
    // of cleanup to be done here, but this actually works

    for (index, group_size) in map_groups.iter().enumerate() {
        // loops through vector of map_groups and tests against map_springs
        // for validity.
        // index enumerator is used to determine group ID further down
        // set size of window to pass over vector to match group size
        let window_size = *group_size;
        let mut window = working_vector.windows(window_size);
        // initialize counter to keep track indices
        let mut counter: usize = 0;
        // initalize outer loop
        'group_loop: loop {
            // set valid_flag to true
            let mut valid_flag: bool = true;
            // start iterator over data
            let active_window = window.next().unwrap();
            // loop over chars inside window, check for uniform validity of
            // symbols in window
            for el in active_window {
                if valid_symbols.contains(el) == false {
                    valid_flag = false;
                    break;
                }
            }
            // check for trailing '#'-es, first check for overflow
            if counter + active_window.len() < bounds
                && working_vector[counter + active_window.len()] == '#'
            {
                valid_flag = false;
            }
            // check for adjacent numbers, also make sure to not underflow
            if counter == 0 {
                if working_vector[counter + active_window.len()].is_numeric() {
                    valid_flag = false;
                }
            } else {
                if working_vector[counter - 1].is_numeric()
                    || working_vector[counter + 1].is_numeric()
                {
                    valid_flag = false;
                }
            }
            // if every test is passed, roll out the group to its correct
            // correct position in the working vector
            if valid_flag {
                // setup for rollout of ID into working vector
                let mut rollout = group_size.clone();
                let group_id = char::from_digit(index.clone() as u32, 10).unwrap();
                loop {
                    if rollout == 0 {
                        // final validity check of working vector
                        if working_vector.contains(&'#') {
                            for el in &working_vector {
                                print!("{}", el);
                            }
                            println!("");
                        }
                        let spring_group = SpringGroup {
                            id: index,
                            size: *group_size,
                            start_index: counter,
                        };
                        groups.push(spring_group);
                        break 'group_loop;
                    }
                    working_vector[counter + rollout - 1] = group_id;
                    rollout -= 1;
                }
            }
            if counter == bounds - window_size {
                break;
            }
            counter += 1;
        }
    }

    //for el in working_vector {
    //    print!("{}", el);
    //}
    //println!("");

    //(working_vector, groups)
}
fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
    // Parses line from full data into types used in:
    // ConditionMap.map_springs and
    // ConditionMap.map_groups

    // crack input string into two parts
    let cracked: Vec<&str> = line.split_whitespace().collect();
    // parse the two output vectors into Vec<char> for spring map
    // and Vec<usize> for group map
    let map_springs: Vec<char> = cracked[0].chars().collect();
    let map_groups: Vec<usize> = cracked[1]
        .split(',')
        .map(|s| s.parse().expect("this should have worked"))
        .collect();
    // return tuple of nice data
    (map_springs, map_groups)
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
