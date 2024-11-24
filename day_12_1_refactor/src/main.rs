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
        let mut spring_groups: Vec<SpringGroup> =  Vec::with_capacity(8);
        for (index, group) in self.groups.iter().enumerate(){
            if index == 0{
                let spring_group = SpringGroup{
                    id: index,
                    size: *group,
                    start_index: 0,
                };
                spring_groups.push(spring_group);
                continue
            }
            let spring_group = SpringGroup{
                id: index,
                size: *group,
                start_index: spring_groups[index - 1].start_index + spring_groups[index - 1].size + 1,
            };
            spring_groups.push(spring_group);
        }
        spring_groups
    }
    
    
}

#[derive(Debug, Clone)]
struct ConditionMap {
    maps: Maps,
    base_arrangement: Vec<char>,
    spring_groups: Vec<SpringGroup>,
}
impl ConditionMap{
    fn build_base_arr(&self) -> Vec<char>{
        let mut output = self.maps.springs.clone();
        for (index, _group) in self.spring_groups.iter().enumerate().clone(){
            output = self.place_group(&index, &output);
        }
    
        output
    }
    fn possibility_printer(&self){
        // First separate out needed variables from self
        
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
    fn lift_group(&self, group_index: &usize, map: &Vec<char>,) -> Vec<char> {
        let reference = self.maps.springs.clone();
        let mut output = map.clone();
        let mut start = self.spring_groups[*group_index].start_index;
        let size = self.spring_groups[*group_index].size;
        let leading_edge = start + size - 1;
        while start <= leading_edge {
            output[start] = reference[start];
            start += 1;
        }
        output
    }
    fn permutator(map:ConditionMap){

    }

}

fn main() {
    let now = Instant::now();
    let path = "./data/day12TT";
    let full_data = get_list_from_file(path);
    for line in full_data{
        starttup(line);
    }
    
    println!("program runtime: {}", now.elapsed().as_micros());
}
fn starttup(line: String){
    let (springs, groups) = parse_line(line);
    let maps = Maps{
        springs, 
        groups, 
    };
    let spring_groups = maps.build_spr_groups();
    let mut condition_map = ConditionMap{
        maps: maps.clone(), 
        base_arrangement: maps.springs.clone(), 
        spring_groups,
    };
    condition_map.base_arrangement = condition_map.build_base_arr();

    println!("{:?}", condition_map.base_arrangement);
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
