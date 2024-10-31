use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

struct Guide {
    directions: Vec<char>,
    map: HashMap<String, (String, String)>,
}

fn main() {
    let now = Instant::now();
    let path = "./data/day8";
    let full_data = get_list_from_file(path);
    let parsed = parser(full_data);
    let path_lengths = pathfinder(parsed);
    let solution = solution_factory(path_lengths);
    println!(
        "Your solution is: {}, and took {} milliseconds to complete",
        solution,
        now.elapsed().as_millis()
    )
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
fn pathfinder(guide: Guide) -> Vec<usize> {
    let directions_len: usize = guide.directions.len() - 1;
    let start_pos = get_starting_pos(&guide.map);
    let mut step_vector: Vec<usize> = Vec::with_capacity(10);
    for el in start_pos {
        let mut steps: usize = 0;
        let mut index: usize = 0;
        let mut position = el;
        print!("{} -> ", position);
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
            if position.chars().last().unwrap() == 'Z' {
                step_vector.push(steps);
                print!("{} in {} steps", position, steps);
                println!(" ");
                break;
            }
        }
    }

    step_vector
}
fn get_starting_pos(map: &HashMap<String, (String, String)>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for (node, _value) in map {
        if node.chars().last().unwrap() == 'A' {
            output.push(node.to_owned());
        }
    }
    output
}
fn step_synchronizer(path_lengths: Vec<usize>) {
    /*Vestigal function, left here to remind myself of the folly of naive
    implementation (still faster than first attempt though.. by about 160
    days) */
    let now = Instant::now();
    let mut output_vec: Vec<usize> = path_lengths.clone();
    let len: usize = path_lengths.len() - 1;
    let mut index: usize = 0;
    loop {
        let mut index2: usize = 0;
        loop {
            if output_vec[index] < output_vec[index2] && index != index2 {
                output_vec[index] = output_vec[index] + path_lengths[index];
            }
            index2 += 1;
            if index2 > len {
                break;
            }
        }
        index += 1;
        if index > len {
            index = 0;
        }
        if output_vec[0] == output_vec[1] {
            if output_vec.iter().min() == output_vec.iter().max() {
                break;
            }
        }
    }
    println!("{} in {} seconds", output_vec[0], now.elapsed().as_secs(),);
}
fn get_factors(mut number: i128) -> Vec<i128> {
    /* Stole this algo from:
    https://rustp.org/number-theory/prime-factorization-of-a-number/
    and modified to output to vector */
    let mut prime_factors: Vec<i128> = Vec::new();

    // Step 1 : Divide by 2
    let mut freq: i128 = 0;

    // You can use number % 2 == 0 also,
    // but this method is much more efficient
    while number & 1 == 0 {
        number >>= 1;
        // Again, You can use number /= 2 also,
        // but this is much more efficient
        freq += 1;
    }

    if freq > 0 {
        prime_factors.push(2);
    }

    // Step 2 : start from 3, and go till square root of number
    let mut i = 3;
    while i * i <= number {
        // Step 3 : Check if i is factor of number
        if number % i == 0 {
            freq = 0;
            while number % i == 0 {
                number /= i;
                freq += 1;
            }
            prime_factors.push(i);
        }
        i += 2;
    }

    // Step 4 : Check if number become 1 or not
    if number > 1 {
        prime_factors.push(number);
    }
    return prime_factors;
}
fn solution_factory(numbers: Vec<usize>) -> i128 {
    let mut all_factors: HashSet<i128> = HashSet::new();
    let mut least_common_multiplier: i128 = 1;
    for el in numbers {
        let number = el as i128;
        let factors = get_factors(number);
        for factor in factors {
            all_factors.insert(factor);
        }
    }
    for factor in all_factors {
        print!("{}, ", factor);
        least_common_multiplier = least_common_multiplier * factor;
    }
    println!("");
    least_common_multiplier
}
