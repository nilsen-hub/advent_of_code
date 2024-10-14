use std::fs::read_to_string;

fn main() {
    let path = "./data/day_4T";
    let full_data = get_list_from_file(path);

    for line in full_data{
        println!("{line}")l
    }
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}