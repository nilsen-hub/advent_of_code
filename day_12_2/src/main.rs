use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct SpringGroup {
    id: usize,
    size: usize,
    start_index: usize,
    si_freedom: usize,
    sh_freedom: usize, 
}
#[derive(Debug, Clone)]
struct Maps {
    springs: Vec<char>,
    groups: Vec<usize>,
}
impl Maps {
    fn build_spr_groups_deprec(&self) -> Vec<SpringGroup> {
        let mut spring_groups: Vec<SpringGroup> = Vec::with_capacity(8);
        for (index, group) in self.groups.iter().enumerate() {
            let id = index;
            let size = *group;
            let mut start_index: usize = 0;
            let si_freedom = 1;
            let sh_freedom = 0
            if index != 0 {
                start_index =
                    spring_groups[index - 1].start_index + spring_groups[index - 1].size + 1;
            }
            let spring_group = SpringGroup {
                id,
                size,
                start_index,
                si_freedom, 
                sh_freedom, 
            };
            spring_groups.push(spring_group);
        }
        spring_groups
    }
    fn build_spr_groups(&self) -> Vec<SpringGroup> {
        // fourth attempt to build a function to make a valid
        // first definition of spring groups, returns a vector
        // of spring group structs
        // The rules for valid spring groups are:
    
        // - Groups must appear in order provided
        // - No groups on '.'
        // - All '#' must be covered by a group
        // - '?' can be filled
        // - No groups may be adjacent to another group
    
        let reference = self.springs.clone();
        let groups = self.groups.clone();
        let mut working_vector = reference.clone();
        let mut spring_groups: Vec<SpringGroup> = Vec::with_capacity(8);
        
        for (index, group) in groups.iter().enumerate(){
            // group ID is current index of groups
            // lets set all known parameters of group
            let id = index;
            let size = *group;
            let si_freedom: usize = 1;
            let sh_freedom: usize = 0;
            let mut start_index: usize = 0;
            // first set the start index of the group
            if index != 0{
                start_index = spring_groups[spring_groups.len() - 1].start_index + spring_groups[spring_groups.len() - 1].size + 1;
            }
            // define head of the group, this together with the
            // start index defines a window
            let mut group_head = start_index + size - 1;

        }

        spring_groups
    }

#[derive(Debug, Clone)]
struct ConditionMap {
    maps: Maps,
    arrangement: Vec<char>,
    spring_groups: Vec<SpringGroup>,
}
impl ConditionMap {
    fn build_arrangement(&self) -> Vec<char> {
        // places spring groups into the reference (conditionMap.maps.springs)
        // and returns the altered map
        let mut output = self.maps.springs.clone();
        for (index, _group) in self.spring_groups.iter().enumerate().clone() {
            output = self.place_group(&index, &output);
        }

        output
    }
    fn arrangement_generator(&mut self) -> Vec<Vec<char>> {
        // generates all possible positions assuming spring groups is only
        // '?', returns vector of possibilites
        // First separate out needed variables from self
        let mut output: Vec<Vec<char>> = Vec::with_capacity(256);
        // fill in starting arrangement
        output.push(self.arrangement.clone());
        let mut spring_groups = self.spring_groups.clone();
        let bounds = self.maps.springs.len();
        // define starting group as len - 1
        let last_group = spring_groups.len() - 1;
        let mut active_group = spring_groups[last_group].clone();
        let mut limit: usize = 0;
        // start loop to move these guys
        'outer: loop {
            //println!("Arrangement generator outer loop started");
            // first, step the last group as far to right as possible, edit self
            // and generate arrangement for each step
            let mut group_head = active_group.start_index + active_group.size;
            limit = bounds;
            active_group.start_index += 1;
            while group_head < limit {
                spring_groups[active_group.id] = active_group.clone();
                self.spring_groups = spring_groups.clone();
                self.arrangement = self.build_arrangement();
                if self.valid_arrangement() {
                    output.push(self.arrangement.clone());
                    self.spring_map_printer();
                }

                active_group.start_index += 1;
                group_head += 1;
            }
            // when last group is all the way to the right, move
            // preceding group one step forward. We're going to toggle
            // the active group marker along the spring_group vector
            // to make things simple and linear. We move down the spring
            // group vector until something can move, and then move it.
            loop {
                // when group 0 cant move anymore, the function is finished
                if active_group.id == 0 {
                    break 'outer;
                }
                active_group = spring_groups[active_group.id - 1].clone();
                // set movement limit for new active group to index
                // right before the next group.
                limit = spring_groups[active_group.id + 1].start_index - 1;
                // define head of current group
                group_head = active_group.start_index + active_group.size - 1;
                // check if group has room to move over once by incrementing
                // start index, copy into spring_group and break loop
                if group_head + 1 < limit {
                    active_group.start_index += 1;
                    spring_groups[active_group.id] = active_group.clone();
                    break;
                }
            }
            // now we need to reset the rest, to get new starting positions.
            // Active group is the last to move, we will move up the spring
            // groups, and fix them one by one
            loop {
                let last_group_head = active_group.start_index + active_group.size - 1;
                active_group = spring_groups[active_group.id + 1].clone();
                if active_group.id == last_group {
                    active_group.start_index = last_group_head + 1;
                    spring_groups[active_group.id] = active_group.clone();
                    break;
                } else {
                    active_group.start_index = last_group_head + 2;
                    spring_groups[active_group.id] = active_group.clone();
                }
            }
        }
        output
    }
    fn spring_map_printer(&self) {
        for el in &self.arrangement {
            print!("{}", el);
        }
        println!("");
    }
    fn place_group(&self, group_index: &usize, map: &Vec<char>) -> Vec<char> {
        // Places one spring group into a map, group index says what
        // group should be placed.
        let mut output = map.clone();
        let mut start = self.spring_groups[*group_index].start_index;
        let id = self.spring_groups[*group_index].id;
        let size = self.spring_groups[*group_index].size;
        let leading_edge = start + size - 1;
        while start <= leading_edge {
            output[start] = '0';
            start += 1;
        }

        output
    }
    fn valid_arrangement(&self) -> bool {
        let arrangement = self.arrangement.clone();
        if arrangement.contains(&'#') {
            return false;
        }
        let reference = self.maps.springs.clone();
        for (index, el) in reference.iter().enumerate() {
            if *el == '.' {
                if arrangement[index] != '.' {
                    return false;
                }
            }
        }

        true
    }
}
fn main() {
    let now = Instant::now();
    let path = "./data/day12T";
    let full_data = get_list_from_file(path);
    let mut value_accumulator: usize = 0;
    for line in full_data {
        value_accumulator += starttup(line);
    }
    println!(
        "There are a total of {} valid permutations in the data set",
        value_accumulator
    );
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn starttup(line: String) -> usize {
    let (springs, groups) = parse_line(line);
    //let (springs, groups) = input_expander(&springs, &groups);
    let maps = Maps { springs, groups };
    let spring_groups = maps.build_spr_groups();
    let mut condition_map = ConditionMap {
        maps: maps.clone(),
        arrangement: maps.springs.clone(),
        spring_groups,
    };
    condition_map.arrangement = condition_map.build_arrangement();
    let permutations = condition_map.arrangement_generator();
    let output = valid_perm_counter(&permutations, &condition_map);

    output
}
fn valid_perm_counter(permutations: &Vec<Vec<char>>, condition_map: &ConditionMap) -> usize {
    let mut condition_map = condition_map.clone();
    let mut valid_counter: usize = 0;
    //println!(
    //    "{:?}  {:?}",
    //    condition_map.maps.springs, condition_map.maps.groups
    //);
    for el in permutations {
        condition_map.arrangement = el.clone();
        if condition_map.valid_arrangement() {
            condition_map.spring_map_printer();
            valid_counter += 1;
        };
    }
    //println!("Contains {} valid arrangement(s)", valid_counter);
    valid_counter
}
fn input_expander(springs: &Vec<char>, groups: &Vec<usize>) -> (Vec<char>, Vec<usize>) {
    let mut big_springs: Vec<char> = Vec::new();
    let mut big_groups: Vec<usize> = Vec::new();
    let mut counter: usize = 5;
    while counter > 0 {
        big_springs.extend(springs.clone());
        if counter > 1 {
            big_springs.push('?');
        }
        counter -= 1;
    }
    counter = 5;
    while counter > 0 {
        big_groups.extend(groups.clone());
        counter -= 1;
    }
    println!("{:?} {:?}", big_springs, big_groups);
    (big_springs, big_groups)
}
fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
    // Parses line from full data into types used in:
    // ConditionMap.map_springs and
    // ConditionMap.map_groups

    // break input string into two parts
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
