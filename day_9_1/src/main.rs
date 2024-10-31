<<<<<<< Updated upstream
use std::{
    fs::read_to_string,
    time::Instant,
};

fn main() {
    let now = Instant::now();
    let path = "./data/day9";
    let full_data = get_list_from_file(path);
    let parsed = parser(full_data);
    let mut value_accumulator = 0;
    for el in parsed{
        value_accumulator += OASIS_predictor(&el);
    }
    println!("The number you are looking for: {}", value_accumulator);

}

fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn parser(full_data:Vec<String>) -> Vec<Vec<i32>> { 
    let mut output: Vec<Vec<i32>> = Vec::new();
    for el in full_data{
        let mut number_vec:Vec<i32> = Vec::new(); 
        let temp = el.split_whitespace();
            for int in temp{
                let int: i32 = match int.parse() {
                    Ok(num) => num,
                    Err(_) => todo!(),
                };
                number_vec.push(int);
            }
        output.push(number_vec);
    }

    output
}
fn OASIS_predictor(data:&Vec<i32>) -> i32{
    // clone data into new vector
    let mut data_vec = data.clone();
    // Assign accumulating vector Vec<Vec<i32>>
    let mut vec_acc: Vec<Vec<i32>> = Vec::new();
    // initialize accumulating vector
    vec_acc.push(data_vec.clone());
    // Assign working vector
    let mut work_vec: Vec<i32> = Vec::new();
    // Assign boundry
    let mut bound = data.len() - 1;
    // Assign index counter
    let mut index: usize = 0;
    // Loop through data vector
    loop{
        //Use index to substract current index from next index in
        let next = data_vec[index + 1] - data_vec[index]; 
        //data vector, push results to working vector
        work_vec.push(next);
        //increment index, check bounds, if at bounds, check data
        index += 1;
        if index == bound{
            //if working vector is all zero, break loop
            if work_vec[0] == 0 && work_vec.iter().min() == work_vec.iter().max(){
                break
            }
            //else push data vector to accumulating vector and assign
            vec_acc.push(work_vec.clone());
            println!("{:?}", work_vec);
            //data vector = work vector
            data_vec = work_vec;
            work_vec = Vec::new();

            // reset index, update bounddry
            index = 0;
            bound -= 1;
        }
    } 
    // Sum last value of all indices in accumulating vector
    let mut sum:i32 = 0;
    for el in vec_acc{
        sum += el[el.len() - 1];
    }
    
    sum
    // return sum
}

mod tests {
    use super::*;

    use crate::OASIS_predictor;
    #[test]
    fn gets_OASIS_predictions(){
        let input_1: Vec<i32> = vec![0, 3, 6, 9, 12, 15];
        let input_2: Vec<i32> = vec![1, 3, 6, 10, 15, 21];
        let input_3: Vec<i32> = vec![10, 13, 16, 21, 30, 45];

        let expected_1: i32 = 18;
        let expected_2: i32 = 28;
        let expected_3: i32 = 68;

        let result_1 = OASIS_predictor(&input_1);
        let result_2 = OASIS_predictor(&input_2);
        let result_3 = OASIS_predictor(&input_3);

        assert_eq!(expected_1, result_1);
        assert_eq!(expected_2, result_2);
        assert_eq!(expected_3, result_3);
    }
}
=======
use std::fs::read_to_string;
fn main() {
    // Get the dataset, return as vector of strings representing the lines of
    // the file
    let path = "./data/day9T";
    let full_parsed_data = 

    // Parse dataset into vector "parsed" of vec<i32>, i32 to account
    // for negative values
    // Pass parsed, index by index, into "predictor" function to do
    // required analysis.
    // Accumulate predictions into value_accumulator and output final
    // result to console.
}

fn get_list_from_file(path: &str) -> Vec<i32> {
    let output:Vec<Vec<i32>> = 
    read_to_string(path)
        .unwrap()
        .lines()
        .map()
        .collect()
}
>>>>>>> Stashed changes
