use std::fs::read_to_string;

#[derive(Debug)]
struct ScratchCard {
    raw_data: String,
    winning_numbers: [bool; 100],
    your_numbers: Vec<usize>,
    wins: u32,
    copies: u32,
}
impl ScratchCard {
    fn get_wins(&self) -> u32 {
        let mut output: u32 = 0;
        for el in &self.your_numbers{
            if self.winning_numbers[*el] == false {
                continue
            } else {
                output += 1;
            }
        }
        output
    }
}

fn main() {
    // grabbing data
    let path = "./data/day_4T";
    let full_data = get_list_from_file(path);
    let mut total_winnings: u32 = 0; 
    for (index, line) in full_data.iter().enumerate(){
        let mut scratchy = build_scratch_card(&line);
        scratchy.wins = scratchy.get_wins();
        println!("Card {} had {} wins on it!", index, scratchy.wins);
        let winnings: u32 = calc_winnings(scratchy.wins);
        total_winnings += winnings
    }

    println!("You win: {} points!", total_winnings);
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn build_scratch_card(raw_data: &str) -> ScratchCard {
    let numbers:Vec<usize> = get_numbers(raw_data); 
    ScratchCard {
        raw_data: raw_data.to_string(),
        winning_numbers: get_winning(&numbers),
        your_numbers: get_yours(&numbers),
        wins: 0,
        copies: 1,
    }
}
fn get_winning(numbers: &Vec<usize>) -> [bool; 100] {
    let mut output: [bool;100] =  [false;100];
    for el in numbers{
        if *el == 999{
            break;
        }
        output[*el] = true;
    }
    output
}
fn get_yours(numbers: &Vec<usize>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
    let mut counter: usize = numbers.len() - 1;
    loop {
        let number: usize = numbers[counter];
        if number == 999 {
            break;
        }
        output.push(number);
        counter -= 1;
    }
    output
}
fn get_numbers(raw_data: &str) -> Vec<usize> {
    let split: Vec<&str> = raw_data.split(":").collect();
    let num_string: &str = &split[1].replace("|", "999");
    let numbers_as_str: Vec<&str> = num_string.split(" ").filter(|&s| !s.is_empty()).collect();
    let mut output: Vec<usize> = Vec::new();
    for el in numbers_as_str {
        let num: usize = el.parse().unwrap();
        output.push(num);
    }
    output
}
fn calc_winnings(wins:u32) -> u32 {
    let mut output:u32 = 1;
    if wins > 1{
        let mut counter:u32 = wins -1;
        while counter != 0{
            output = output << 1;
            counter -= 1;
        }
    }else{
        output = wins;
    }
    output
}
