use std::{fs::read_to_string, time::Instant};

#[derive(Debug, Clone)]
struct Maps {
    springs: Vec<char>,
    groups: Vec<usize>,
}
#[derive(Debug, Clone)]
struct Group {
    id: usize,
    size: usize,
    start_index: usize,
}

fn main() {
    let now = Instant::now();
    let path = "./data/day12T";
    let full_data = get_list_from_file(path);
    for line in full_data {
        let (springs, groups) = parse_line(line);
        let maps = Maps { springs, groups };

        println!("{:?}", maps.springs);
    }
    println!("program runtime: {}", now.elapsed().as_micros());
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
