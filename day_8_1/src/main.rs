use std::{collections::HashMap, fs::read_to_string};

struct Guide {
    directions: Vec<char>,
    map: HashMap<String, (String, String)>,
}

fn main() {
    let path = "./data/day8";
    let full_data = get_list_from_file(path);
    let parsed = parser(full_data);
    pathfinder(parsed);
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn parser(full_data: Vec<String>) -> Guide {
    let directions: Vec<char> = full_data[0].chars().collect();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for (index, line) in full_data.iter().enumerate() {
        if index < 2 {
            continue;
        }
        let node = line[0..3].to_owned();
        let left_right = (line[7..10].to_owned(), line[12..15].to_owned());
        map.insert(node, left_right);
    }
    Guide { directions, map }
}
fn pathfinder(guide: Guide) {
    let directions_len: usize = guide.directions.len() - 1;
    let mut position: String = String::from("VPA");
    let end: String = String::from("GGZ ");
    let mut steps_since: usize = 0;
    let mut steps: usize = 0;
    let mut index: usize = 0;
    loop {
        let forward = guide.map.get(&position).unwrap();
        if guide.directions[index] == 'L' {
            position = forward.0.clone();
        } else {
            position = forward.1.clone();
        }
        if index == directions_len {
            index = 0;
        } else {
            index += 1;
        }
        steps += 1;
        steps_since += 1;
        if position == end {
            println!("{}, {}", steps, steps_since);
            steps_since = 0;
        }
    }
    println!("{steps}");
}
