use std::{collections::VecDeque, fs::read_to_string, iter::Map, time::Instant};

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
    let path = "./data/day12T";
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
    // -> Vec<SpringGroups>, base_arrangement (Vec<char>)
    // The third time I'm attempting to build a decent function for this..
    // Takes maps, and uses them to build a valid base arrangement of
    // spring groups in the spring maps, also returns a vector of
    // spring group coordinates.

    // Builds a valid base case of spring groups
    // The rules are 
    
    // - Groups must appear in order provided
    // - No groups on '.'
    // - All '#' must be covered by a group
    // - '?' can be filled
    // - No groups may be adjacent to another group

    // this time around, to give some extra safety, i will consume
    // placed groups into a new vector as I go, this should eliminate
    // some errors encountered in the previous two attempts
    // Actual function code and comments start below here:
    // First set up a working enviroment and some helpful constants
    // Reference, to have a clean map to compare to
    // working vector to do process and analyze
    // Vector to collect placed groups
    // Also make groups variable, for semantic clarity
    // Have valid symbols ready
    // Instantiating vector which will hold the finished SpringGroup structs
    let reference = maps.springs;
    let mut working_vector = reference.clone();
    let mut output_vector: Vec<char> = Vec::with_capacity(30);
    let groups = maps.groups;
    let valid_symbols: [char; 2] = ['?','#'];
    let mut output: Vec<SpringGroup> = Vec::with_capacity(8);
    println!("{:?}", reference);

    for (index, group_size) in groups.iter().enumerate(){
        // loops through groups, checks them against reference, and
        // eventually transfers valid groups from working vector into
        // output vector. Index is used as group ID in SpringGroup struct
        // Define window matching size of current group to pass over 
        // working Vector. 
        // Instantiate counter for indexing.
        // Instantiate bounds to avoid overflows
        // Make group amount variable to help semantic clarity
        // and start loop. 
        let mut window = working_vector.windows(*group_size);
        let mut counter: usize = 0;
        let bounds = working_vector.len();
        let next_index = counter + group_size;
        let group_amount = groups.len();
        'group_loop: loop{
            let mut valid_flag:bool = true;
            // start or advance window
            let active_window = window.next().unwrap();
            // check that window contains all valid symbols
            for el in active_window{
                if valid_symbols.contains(el) == false{
                    valid_flag = false;
                    break;
                }
            }
            // check for trailing '#', also check your not looking out of bounds
            if next_index < bounds && working_vector[next_index] == '#'{
                valid_flag = false;
            }
            // if both checks pass, the group has a valid placement.
            // Insert group into working vector, split group and next index
            // (if available) away from working vector, and append to 
            // output_vector
            if valid_flag {
                if index + 1 == group_amount{
                    output_vector.append(&mut working_vector);
                } else {
                    let (left, right) = working_vector.split_at(next_index + 1);
                    for el in left {
                        output_vector.push(*el);
                    }
                    working_vector = (*right).to_vec();
                }
                println!("{:?}", output_vector);
                break 'group_loop;

            }
        }
    }
    // println!("{:?}", working_vector);
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
