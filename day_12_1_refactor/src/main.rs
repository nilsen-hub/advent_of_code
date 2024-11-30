use std::{fs::read_to_string, time::Instant};

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
impl Maps {
    fn build_spr_groups(&self) -> Vec<SpringGroup> {
        let mut spring_groups: Vec<SpringGroup> = Vec::with_capacity(8);
        for (index, group) in self.groups.iter().enumerate() {
            let id = index;
            let size = *group;
            let mut start_index: usize = 0;
            if index != 0 {
                start_index =
                    spring_groups[index - 1].start_index + spring_groups[index - 1].size + 1;
            }
            let spring_group = SpringGroup {
                id,
                size,
                start_index,
            };
            spring_groups.push(spring_group);
        }
        spring_groups
    }
}

#[derive(Debug, Clone)]
struct ConditionMap {
    maps: Maps,
    arrangement: Vec<char>,
    spring_groups: Vec<SpringGroup>,
}
impl ConditionMap {
    fn build_arrangement(&self) -> Vec<char> {
        let mut output = self.maps.springs.clone();
        for (index, _group) in self.spring_groups.iter().enumerate().clone() {
            output = self.place_group(&index, &output);
        }
        output
    }
    fn arrangement_generator(&mut self) -> Vec<Vec<char>> {
        let mut output: Vec<Vec<char>> = Vec::with_capacity(256);
        if self.valid_arrangement() {
                output.push(self.arrangement.clone());
            }
        let mut spring_groups = self.spring_groups.clone();
        let bounds = self.maps.springs.len();
        let last_group = spring_groups.len() - 1;
        let mut active_group = spring_groups[last_group].clone();
        let mut counter: usize = 0;
        let mut metacounter: usize = 0;
        'outer: loop {
            counter += 1;
            let mut group_head = active_group.start_index + active_group.size;
            let mut limit = bounds;
            active_group.start_index += 1;
            while group_head < limit {
                counter += 1;
                spring_groups[active_group.id] = active_group.clone();
                self.spring_groups = spring_groups.clone();
                self.arrangement = self.build_arrangement();
                if self.valid_arrangement() {
                    output.push(self.arrangement.clone());
                    //if counter == 10_000_000{
                    //    metacounter += 1;
                    //    counter = 0;
                    //    println!("ten million iterations times {}", metacounter);
                    //}
                }
                active_group.start_index += 1;
                group_head += 1;
            }
            loop {
                if active_group.id == 0 {
                    break 'outer;
                }
                active_group = spring_groups[active_group.id - 1].clone();
                limit = spring_groups[active_group.id + 1].start_index - 1;
                group_head = active_group.start_index + active_group.size - 1;
                if group_head + 1 < limit {
                    active_group.start_index += 1;
                    spring_groups[active_group.id] = active_group.clone();
                    break;
                }
            }
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
    fn place_group(&self, group_index: &usize, map: &Vec<char>) -> Vec<char> {
        let mut output = map.clone();
        let mut start = self.spring_groups[*group_index].start_index;
        let id = self.spring_groups[*group_index].id;
        let size = self.spring_groups[*group_index].size;
        let leading_edge = start + size - 1;
        while start <= leading_edge {
            output[start] = char::from_digit(id.clone() as u32, 10).unwrap();
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
            if *el == '.' && arrangement[index] != '.' {
                return false;
            }
        }
        true
    }
}
fn main() {
    let now = Instant::now();
    let path = "./data/day12";
    let full_data = get_list_from_file(path);
    let mut value_accumulator: usize = 0;
    for line in full_data {
        value_accumulator += starttup(line);
    }
    println!(
        "There is a total of {} valid permutation(s) in the data set",
        value_accumulator
    );
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn starttup(line: String) -> usize {
    let (springs, groups) = parse_line(line);
    let maps = Maps { springs, groups };
    let spring_groups = maps.build_spr_groups();
    let mut condition_map = ConditionMap {
        maps: maps.clone(),
        arrangement: maps.springs.clone(),
        spring_groups,
    };
    condition_map.arrangement = condition_map.build_arrangement();
    let permutations = condition_map.arrangement_generator();
    let output = valid_perm_counter(&permutations);
    output
}
fn valid_perm_counter(permutations: &Vec<Vec<char>>) -> usize {
    let valid_counter = permutations.len();
    valid_counter
}
fn parse_line(line: String) -> (Vec<char>, Vec<usize>) {
    let cracked: Vec<&str> = line.split_whitespace().collect();
    let map_springs: Vec<char> = cracked[0].chars().collect();
    let map_groups: Vec<usize> = cracked[1]
        .split(',')
        .map(|s| s.parse().expect("this should have worked"))
        .collect();
    (map_springs, map_groups)
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}