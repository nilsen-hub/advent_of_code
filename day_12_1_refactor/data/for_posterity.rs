use std::{char, fs::read_to_string, iter::Enumerate, ops::Index, time::Instant};

#[derive(Debug, Clone)]
struct SpringGroup {
    id: usize,
    size: usize,
    start_index: usize,
}
#[derive(Debug, Clone)]
struct ConditionMap {
    map_springs: Vec<char>,
    map_groups: Vec<usize>,
    first_arrangement: Vec<char>,
    spring_groups: Vec<SpringGroup>,
}

fn main() {
    let now = Instant::now();
    let path = "./data/day12T";
    let full_data = get_list_from_file(path);
    for line in full_data {
        let condition_map = build_condition_map(line);
        condition_map_analysis(&condition_map);
    }

    println!("program runtime: {}", now.elapsed().as_micros());
}
fn condition_map_analysis(map: &ConditionMap) {
    // -> usize
    // we want to analyze the maps in reverse, as they are left leaning
    // we want to check from the right if there is avaliable movement
    // I plan to refactor out unnescesary clones, but for now it lowers
    // cognitive overhead
    // setup shorthand
    let bounds = map.map_springs.len();
    let mut step_counter: usize = 0;
    let reference = &map.map_springs;
    let mut working_map = map.first_arrangement.clone();
    let mut groups = map.spring_groups.clone();
    groups.reverse();
    // check freedom of motion in all groups (and groups of groups).
    // first, check if group is locked with a '#' at either end, remove
    // locked groups from vector
    let mut counter: usize = 0;
    loop {
        if counter == groups.len() {
            break;
        }
        let start = reference[groups[counter].start_index];
        let end = reference[groups[counter].start_index + groups[counter].size - 1];
        if start == '#' || end == '#' {
            groups.remove(counter);
            // this bit is horribly unoptimized, I'll leave it for now
            counter = 0;
            continue;
        }
        counter += 1;
    }
    // checks if there is more than one valid arrangement to be had
    for group in groups {
        // it feels kinda like this code is on the brink of panicing..
        if group.start_index + group.size < bounds {
            if working_map[group.start_index + group.size] == '?'
                && working_map[group.start_index + group.size + 1].is_numeric() == false
            {
                step_counter += 1;
                break;
            }
        }
    }
    // if no steps are counted, return 1
    // if step_counter == 0{
    // return 1
    //}
    // Check for linked groups
    // if there are no linked groups

    // debug print: for el in reference{
    // debug print:     print!("{} ", el);
    // debug print: }
    // debug print: if step_counter == 0{
    // debug print:     print!("Has only one valid arrangement");
    // debug print: } else {
    // debug print:     print!("Has more than one valid arrangement");
    // debug print: }
    // debug print: println!("");
}
fn build_condition_map(line: String) -> ConditionMap {
    //
    // build instance of ConditionMap
    // use parse_line() to get map_springs and map_groups
    // map_groups might be redundant in the final ConditionMap, as we have the
    // SpringGroup struct with enriched data on spring groups, we still need
    // the data though, to build the SpringGroup struct
    let (map_springs, map_groups) = parse_line(line);
    // get first valid arrangement of spring groups in spring maps
    // get spring groups vector as well
    let (first_arrangement, spring_groups) = parse_maps(&map_springs, &map_groups);
    let condition_map = ConditionMap {
        map_springs,
        map_groups,
        first_arrangement,
        spring_groups,
    };
    condition_map
}
fn parse_maps(map_springs: &Vec<char>, map_groups: &Vec<usize>) -> (Vec<char>, Vec<SpringGroup>) {
    // BUG UNCOVERED
    // NEED TO COVER THIS CASE: ??.??????.?#????? 1,4
    // set up definitions and workspace
    // clone map_springs to have a workplace, and a clean reference
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
                        let spring_group = SpringGroup {
                            id: group_id as usize - 48, //hacky ASCII shift
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

    (working_vector, groups)
}
fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
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
fn to_refactor() {
    // set up data and structures
    let string: &str = "????????????????????";
    let groups: Vec<usize> = vec![1, 5, 6, 3];
    let chars: Vec<char> = string.chars().collect();
    // set up definitions and working area
    let mut working_vector: Vec<char> = chars.clone();
    let bounds: usize = chars.len();
    let valid_symbols: [char; 2] = ['?', '#'];

    // Placement rules:
    // '#' MUST be filled
    // '?' might be filled
    // no placement on '.' characters
    // no two groups on adjacent indices

    for (index, group_size) in groups.iter().enumerate() {
        // debug println!("Currently working on index: {}", index);
        // loops through vector of group sizes and makes initial placement
        // of groups in working array. The index is used to name groups
        // after their index in the groups vector

        // set window size to correct group size
        let window_size = *group_size;
        let mut window = working_vector.windows(window_size);
        // initalize counter to keep track of indices
        let mut counter: usize = 0;
        'group_loop: loop {
            // checks if all elements in window are possible valid insertion sites
            // assumes true until proven otherwise
            let mut valid_flag: bool = true;
            let active_window = window.next().unwrap();
            for el in active_window {
                if valid_symbols.contains(el) == false {
                    valid_flag = false;
                    break;
                }
            }
            // accounts for consecutive '#', disambiguates placement issues
            // if the next character outside window is '#', set valid flag
            // to false
            // first check for vector overflow
            if counter + active_window.len() < bounds {
                if working_vector[counter + active_window.len()] == '#' {
                    valid_flag = false;
                }
            }

            // if symbol flag is true, check if there are adjacent groups
            // to invalidate placements, account for 0 index-shenanigans
            // since inital placements are "left leaning", you only need to
            // check index - 1

            // this accounts for 0 index shenaningans
            // if the window starting at the first index of window_vector
            // is valid this loop fills the appropriate indices with
            // correct numerical data

            if valid_flag
                && counter == 0
                && working_vector[counter + active_window.len()].is_numeric() == false
            {
                let mut rollout: usize = group_size.clone();
                loop {
                    if rollout == 0 {
                        // debug println!("Currently finishing up on index: {}", index);
                        break 'group_loop;
                    }
                    let group_id = char::from_digit(index.clone() as u32, 10).unwrap();
                    working_vector[counter + (rollout - 1)] = group_id;
                    rollout -= 1;
                }
            }

            // this should handle the rest of cases. Checks index here aswell
            // to avoid handling index 0 twice.

            if valid_flag
                && counter > 0
                && working_vector[counter - 1].is_numeric() == false
                && working_vector[counter + 1].is_numeric() == false
            {
                let mut rollout: usize = group_size.clone();
                loop {
                    if rollout == 0 {
                        println!("Currently finishing up on index: {}", index);
                        break 'group_loop;
                    }
                    let group_id = char::from_digit(index.clone() as u32, 10).unwrap();
                    working_vector[counter + (rollout - 1)] = group_id;
                    rollout -= 1;
                }
            }

            if counter == bounds - window_size {
                break;
            }
            counter += 1;
        }
    }

    for c in working_vector {
        print!("{c} ");
    }
    println!("");
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

mod tests {
    use super::*;
    #[test]
    fn calculates_distance() {
        let time = 7;
        let push_time = 2;
        let result = distance_calculator(time, push_time);
        let expected: usize = 10;
        assert_eq!(result, expected);
    }
}
