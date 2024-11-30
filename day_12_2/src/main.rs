use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct SpringGroup {
    id: usize,
    size: usize,
    start_index: (usize, usize),
}
#[derive(Debug, Clone)]
struct Constellation {
    groups: Vec<SpringGroup>,
    freedoms: usize,
}

#[derive(Debug, Clone)]
struct Maps {
    springs: Vec<char>,
    groups: Vec<usize>,
}
impl Maps {
    fn get_fragments(&self) ->Vec<Vec<char>> {
        let mut fragments: Vec<Vec<char>> = Vec::with_capacity(16);
        let mut fragment: Vec<char> = Vec::with_capacity(128);
        let springs = self.springs.clone();
        
        for el in springs{
            if el == '.'{
                if fragment.len() > 0{
                    fragments.push(fragment.clone());
                }
                fragment.clear();
                continue;
            }
            fragment.push(el);
        }
        if fragment.len() > 0{
            fragments.push(fragment.clone());
        }

        fragments
    }
}

#[derive(Debug, Clone)]
struct ConditionMap {
    maps: Maps,
    constellations: Vec<Constellation>,
    fragments: Vec<Vec<char>>,
    hashes: Vec<(usize, usize)>,
    populated: Vec<Vec<char>>,
}
impl ConditionMap {
    fn get_hashes(&self) ->Vec<(usize, usize)> {
        let mut hashes: Vec<(usize, usize)> = Vec::with_capacity(16);
        for (index, fragment) in self.fragments.iter().enumerate(){
            for (idx, char) in fragment.iter().enumerate(){
                if *char == '#'{
                    let hash = (index, idx);
                    hashes.push(hash.clone());
                }
            }
        }   
        hashes
    }
    fn get_first_constellation(&mut self){
        // Function to generate constellations.
        // A constellation is a configuration of groups within the given fragments
        // An exhaustive list of constellations are required for an accurate
        // count of arrangements. I aspired to doing this mostly by modifying and 
        // checking numbers, but we will see if we have to do some construction

        let groups = self.maps.groups.clone();
        let fragments = self.fragments.clone();
        let mut spring_groups: Vec<SpringGroup> = Vec::new();
        for el in fragments{
            println!("{:?}", el);
        }
        // Make first constellation by throwing the springgroups into their
        // first suitable spot.
        for (index, group) in groups.iter().enumerate(){
            let start_index = self.get_start_index(group, &spring_groups);
            let spring_group = SpringGroup{
                id: index,
                size: *group,
                start_index,
            };
            spring_groups.push(spring_group.clone());
        }
        // build constellation with placeholder freedoms and push it onto self
        let constellation = Constellation{
            groups: spring_groups,
            freedoms: 1,
        }; 

        self.constellations.push(constellation);

        // Validate constellation, if not valid, fix it
        if self.valid_constellation() == false{
            println!("Not a valid constellation");
            self.constellation_fixer();
        }
        self.constellation_printer();
    }
    fn constellation_fixer(&mut self){
        // fixes the cases where one or more hashes are left uncovered by the
        // naive first placement of groups in a constellation.
        // works by identifying the closest group to the left of the exposed
        // hash, and moving it. Checks for adjacency need to be in place aswell

        // first get a list of exposed hashes.
        let to_fix = self.get_open_hashes();
        let mut group_to_move: usize = 0;

        // go through list of exposed hashes
        for fix in to_fix{
            // identify spring group to the left of hash find the first group with
            // a start index higher than the hash, and take the group before it.
            // if last group has start index lower than hash, that group needs to move
            let start_index_last_group = self.constellations[0].groups[self.constellations[0].groups.len() - 1].start_index;
            if start_index_last_group.0 <= fix.0 && start_index_last_group.1 < fix.1{
                group_to_move = self.constellations[0].groups[self.constellations[0].groups.len() - 1];
            } else{
                for (index, group) in self.constellations[0].groups.iter().enumerate(){
                 let start_index = group.start_index;
                 if start_index.0 >= fix && start_index.1 > fix.1{
                    group_to_move = self.constellations[0].groups[index - 1];
                 }
                }
            }
        }

    }
    fn get_open_hashes(&self) -> Vec<(usize, usize)>{
        let mut output: Vec<(usize, usize)> = Vec::with_capacity(16);
        for hash in &self.hashes{
            let  mut check = false;
            for spring_group in &self.constellations[0].groups{
                let start = spring_group.start_index;
                let coverage = start.1 + spring_group.size - 1;
                // println!("start: {:?}  coverage: {}  hash: {:?}", start, coverage, hash);
                if hash.0 != start.0{
                    continue;
                }
                if start.1 <= hash.1 && coverage >= hash.1{
                    // println!("its true!");
                    check = true;
                }
            }
            if check == false{
                output.push(*hash);
            }
        }
        output
    }
    fn constellation_printer(&mut self){

        for (index, el) in self.constellations.clone().iter().enumerate(){
            for group in el.groups.clone(){
                self.place_group(group.id, index);
            }
        }
        for el in &self.populated{
            for char in el{
                print!("{}", char);
            }
            print!("  ");
        }
        println!("");

    }
    fn place_group(&mut self, group_id: usize, constellation_number: usize){
        
        let mut fragments = self.populated.clone();
        let mut size = self.constellations[constellation_number].groups[group_id].size;
        
        while size > 0{
            let placement_index = self.constellations[constellation_number].groups[group_id].start_index.1 + size - 1;
            let fragment = self.constellations[constellation_number].groups[group_id].start_index.0;
            fragments[fragment][placement_index] = 'X';
            size -= 1;
        }

        self.populated = fragments.clone();
    }
    fn valid_constellation(&self) -> bool{
        for hash in &self.hashes{
            let  mut output = false;
            for spring_group in &self.constellations[0].groups{
                let start = spring_group.start_index;
                let coverage = start.1 + spring_group.size - 1;
                // println!("start: {:?}  coverage: {}  hash: {:?}", start, coverage, hash);
                if hash.0 != start.0{
                    continue;
                }
                if start.1 <= hash.1 && coverage >= hash.1{
                    // println!("its true!");
                    output = true;
                }
            }
            if output == false{
                return false
            }
        }

        return true
    }
    fn get_start_index(&self, group: &usize, spring_groups:&Vec<SpringGroup>) ->(usize, usize) {
        // gets starting index for a group
        let spring_groups = spring_groups.clone();
        let mut output: (usize, usize) = (0,0);
        let mut fragment_counter: usize = 0;
        let mut cell_counter: usize = 0;
        let fragments = self.fragments.clone();
        
        // sets counters to engage after last placed group.
        if spring_groups.len() > 0{
            let last_group = spring_groups[spring_groups.len() - 1].clone();
            (fragment_counter, cell_counter) = last_group.start_index;
            cell_counter += last_group.size + 1;
        }
        'fragments_loop: loop{
            let fragment = fragments[fragment_counter].clone();
            loop{
                let group_counter = cell_counter + group - 1;
                if group_counter >= fragment.len(){
                    break;
                }
                output = (fragment_counter, cell_counter);
                break 'fragments_loop;
            }
            cell_counter = 0;
            fragment_counter += 1;
        }
        output
    }

}
// impl ConditionMap {
//     fn build_arrangement(&self) -> Vec<char> {
//         // places spring groups into the reference (conditionMap.maps.springs)
//         // and returns the altered map
//         let mut output = self.maps.springs.clone();
//         for (index, _group) in self.spring_groups.iter().enumerate().clone() {
//             output = self.place_group(&index, &output);
//         }
// 
//         output
//     }
//     fn arrangement_generator(&mut self) -> Vec<Vec<char>> {
//         // generates all possible positions assuming spring groups is only
//         // '?', returns vector of possibilites
//         // First separate out needed variables from self
//         let mut output: Vec<Vec<char>> = Vec::with_capacity(256);
//         // fill in starting arrangement
//         output.push(self.arrangement.clone());
//         let mut spring_groups = self.spring_groups.clone();
//         let bounds = self.maps.springs.len();
//         // define starting group as len - 1
//         let last_group = spring_groups.len() - 1;
//         let mut active_group = spring_groups[last_group].clone();
//         let mut limit: usize = 0;
//         // start loop to move these guys
//         'outer: loop {
//             //println!("Arrangement generator outer loop started");
//             // first, step the last group as far to right as possible, edit self
//             // and generate arrangement for each step
//             let mut group_head = active_group.start_index + active_group.size;
//             limit = bounds;
//             active_group.start_index += 1;
//             while group_head < limit {
//                 spring_groups[active_group.id] = active_group.clone();
//                 self.spring_groups = spring_groups.clone();
//                 self.arrangement = self.build_arrangement();
//                 if self.valid_arrangement() {
//                     output.push(self.arrangement.clone());
//                     self.spring_map_printer();
//                 }
// 
//                 active_group.start_index += 1;
//                 group_head += 1;
//             }
//             // when last group is all the way to the right, move
//             // preceding group one step forward. We're going to toggle
//             // the active group marker along the spring_group vector
//             // to make things simple and linear. We move down the spring
//             // group vector until something can move, and then move it.
//             loop {
//                 // when group 0 cant move anymore, the function is finished
//                 if active_group.id == 0 {
//                     break 'outer;
//                 }
//                 active_group = spring_groups[active_group.id - 1].clone();
//                 // set movement limit for new active group to index
//                 // right before the next group.
//                 limit = spring_groups[active_group.id + 1].start_index - 1;
//                 // define head of current group
//                 group_head = active_group.start_index + active_group.size - 1;
//                 // check if group has room to move over once by incrementing
//                 // start index, copy into spring_group and break loop
//                 if group_head + 1 < limit {
//                     active_group.start_index += 1;
//                     spring_groups[active_group.id] = active_group.clone();
//                     break;
//                 }
//             }
//             // now we need to reset the rest, to get new starting positions.
//             // Active group is the last to move, we will move up the spring
//             // groups, and fix them one by one
//             loop {
//                 let last_group_head = active_group.start_index + active_group.size - 1;
//                 active_group = spring_groups[active_group.id + 1].clone();
//                 if active_group.id == last_group {
//                     active_group.start_index = last_group_head + 1;
//                     spring_groups[active_group.id] = active_group.clone();
//                     break;
//                 } else {
//                     active_group.start_index = last_group_head + 2;
//                     spring_groups[active_group.id] = active_group.clone();
//                 }
//             }
//         }
//         output
//     }
//     fn spring_map_printer(&self) {
//         for el in &self.arrangement {
//             print!("{}", el);
//         }
//         println!("");
//     }
//     fn place_group(&self, group_index: &usize, map: &Vec<char>) -> Vec<char> {
//         // Places one spring group into a map, group index says what
//         // group should be placed.
//         let mut output = map.clone();
//         let mut start = self.spring_groups[*group_index].start_index;
//         let id = self.spring_groups[*group_index].id;
//         let size = self.spring_groups[*group_index].size;
//         let leading_edge = start + size - 1;
//         while start <= leading_edge {
//             output[start] = '0';
//             start += 1;
//         }
// 
//         output
//     }
//     fn valid_arrangement(&self) -> bool {
//         let arrangement = self.arrangement.clone();
//         if arrangement.contains(&'#') {
//             return false;
//         }
//         let reference = self.maps.springs.clone();
//         for (index, el) in reference.iter().enumerate() {
//             if *el == '.' {
//                 if arrangement[index] != '.' {
//                     return false;
//                 }
//             }
//         }
// 
//         true
//     }
// }
fn main() {
    let now = Instant::now();
    let path = "./data/day12TT";
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
    let output = 1;
    let (springs, groups) = parse_line(line);
    //let (springs, groups) = input_expander(&springs, &groups);
    let maps = Maps { springs, groups };
    let fragments = maps.get_fragments();
    let constellations:Vec<Constellation> = Vec::new();
    let hashes: Vec<(usize, usize)> = Vec::new();
    let populated = fragments.clone(); 
    let mut condition_map = ConditionMap {
        maps,
        constellations,
        fragments,
        hashes,
        populated,
    };
    condition_map.hashes = condition_map.get_hashes();
    condition_map.get_first_constellation();


    output
}

//fn valid_perm_counter(permutations: &Vec<Vec<char>>, condition_map: &ConditionMap) -> usize {
//    let mut condition_map = condition_map.clone();
//    let mut valid_counter: usize = 0;
//    //println!(
//    //    "{:?}  {:?}",
//    //    condition_map.maps.springs, condition_map.maps.groups
//    //);
//    for el in permutations {
//        condition_map.arrangement = el.clone();
//        if condition_map.valid_arrangement() {
//            condition_map.spring_map_printer();
//            valid_counter += 1;
//        };
//    }
//    //println!("Contains {} valid arrangement(s)", valid_counter);
//    valid_counter
//}
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
