// Advent of code 2023 day 3 part 1, kernels and stuff.

use std::fs::read_to_string;

fn main() {
    let mut value_accumulator: u32 = 0;
    let path = "./data/day_3_1";
    let full_data = get_list_from_file(path);
    let row_len = get_row_length(&full_data);
    let flattened = vector_cat(&full_data);
    let boundry = flattened.len();
    let mut count_index = 0;
    
    loop{
        let c = flattened[count_index];
        if c.is_numeric(){
            let prt_number = number_cat(count_index, &flattened, row_len);
            let is_part = kernel_analysis(&prt_number, &flattened, row_len, boundry);
            for c in &prt_number{
                print!("{}", &flattened[count_index]);
                count_index += 1;
            }
            if is_part{
                value_accumulator += make_number(prt_number, &flattened);
                print!(" is a part!");
                println!(" ");
            } else {
                print!(" is not a part");
                println!(" ");
            }
        } else{
            count_index += 1;
        }
        if count_index == boundry{
            break
        }
    }
    println!("The sum of all parts: {}", value_accumulator);
}

fn make_number(input:Vec<usize>, data:&Vec<char>) -> u32{
    let mut num_string = String::new();
    for el in input{
        num_string.push(data[el]);
    }
    let output = num_string.parse::<u32>().unwrap();
    output
}

fn kernel_analysis(nums:&Vec<usize>, data:&Vec<char>, len:usize, boundry:usize) -> bool {
    for i in nums{
        if is_edge(i, len, boundry){
            let kernel:Vec<usize> = edge_demystifier(*i, len, boundry);
            for e in kernel{
                if is_symbol(data[e]){
                    return true
                }
            }
        } else {
            let kernel:Vec<usize> = vec![
                i-1,
                i-(len - 1),
                i-len,
                i-(len + 1),
                i+1,
                i+(len-1),
                i+len,
                i+(len+1)];

                for e in kernel{
                    if is_symbol(data[e]){
                        return true
                    }
                }
            }
        }

    false
}
// must be poosible to do this with match, its late and im tired
fn edge_demystifier(i:usize, len:usize, boundry:usize) -> Vec<usize>{
    if i == 0{
        let top_left:Vec<usize> = vec![i+1, i+len, i+(len+1)];
        return top_left
    }
    if (i as i32) - (len as i32) < 0 && i % len == len - 1{
        let top_right:Vec<usize> = vec![i-1, i+(len-1), i+len];  
        return top_right
    }
    if i % len == 0 && i + len >= boundry{
        let bottom_left:Vec<usize> = vec![i+1, i-len, i-(len+1)]; 
        return bottom_left
    }
    if i + 1 >= boundry{
        let bottom_right:Vec<usize> = vec![i-1, i-len, i-(len-1)];
        return bottom_right
    }
    if (i as i32) - (len as i32) < 0{
        let top:Vec<usize> = vec![i-1, i+1, i+(len-1), i+len, i+(len+1)];
        return top
    }
    if i % len == 0{
        let left:Vec<usize> = vec![i-len, i-(len+1), i+1, i+len, i+(len+1)];
        return left
    }
    if i % len == len - 1{
        let right:Vec<usize> = vec![i-1, i-(len-1), i-len, i+(len-1), (i+len)];
        return right
    }
    if i + len > boundry{
        let bottom:Vec<usize> = vec![i-1, i-(len-1), i-len, i-(len+1), i+1];
        return bottom
    } else {
        let error:Vec<usize> = vec![0,0,0,0,0,0,0,0,0,0,0,0,0];
        println!("ERRORERRORERROR");
        return error
    }
    //let cases:Vec<bool> = vec![
    //    index == 0,
    //    (*index as i32) - (len as i32) < 0 && index % len == len - 1,
    //    index % len == 0 && index + len >= boundry,
    //    index + 1 == boundry,
    //    (*index as i32) - (len as i32) < 0,
    //    index % len == 0,
    //    index % len == len - 1,
    //    index + len > boundry
    //]

}

fn number_cat(index: usize, data: &Vec<char>, len: usize) -> Vec<usize> {
    let mut counter:usize = index;
    let mut output: Vec<usize> = Vec::new();
    loop{
        if data[counter].is_numeric(){
            output.push(counter);
            if counter % len == len - 1{
                break
            }
        } else {
            break
        }
        counter += 1;
    }
    output
}

fn get_list_from_file(path: &str) -> Vec<String>{
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_row_length(input: &Vec<String>) -> usize {
    let mut output = 0;
    for el in input{
        output = el.len();
        break
    }
    return output
}

fn vector_cat(input: &Vec<String>) -> Vec<char> {
    let mut output: Vec<char> = Vec::new();
    for el in input{
        let my_chars: Vec<_> = el.chars().collect();
        for el in my_chars{
            output.push(el);
        }
    }
    return output
}

fn is_edge(index: &usize, len:usize, boundry:usize) -> bool {
    let edge_list:Vec<bool> = vec![
        (*index as i32) - (len as i32) < 0,
        index % len == 0,
        index % len == len - 1,
        index + len > boundry];

        for e in edge_list{
            if e{
                return true
            }
        }
    
    false
}

fn is_symbol(input:char) -> bool {
    if input == '.' || input.is_numeric(){
        return false
    }
    true
}
