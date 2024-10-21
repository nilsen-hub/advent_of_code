use std::{fs::read_to_string, i64, iter::Enumerate};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<i64>,
    seed_soil: Vec<(i64, i64, i64)>,
    soil_fertilizer: Vec<(i64, i64, i64)>,
    fertilizer_water: Vec<(i64, i64, i64)>,
    water_light: Vec<(i64, i64, i64)>,
    light_temp: Vec<(i64, i64, i64)>,
    temp_humidity: Vec<(i64, i64, i64)>,
    humidity_locaction: Vec<(i64, i64, i64)>,
}

fn main() {
    let path = "./data/day_5T";
    let full_data = get_list_from_file(path);
    let almanac = farmer(&full_data);
    println!("{} is the closest location!", find_lowest_location(&almanac));
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn farmer(data: &Vec<String>) -> Almanac{
    /*The farmer, write_almanac, takes the complete dataset and uses the farmers 
    machines to get it all neat and orderly, returns the neatest, most orderly 
    almanac you ever saw */

    let bounds = data.len();
    let bootstrap = seed_separator(0, data);
    let seeds = bootstrap.1;
    let mut parser:(usize, Vec<(i64, i64, i64)>) = (bootstrap.0, Vec::with_capacity(50));
    let mut maps:[Vec<(i64, i64, i64)>; 7] = Default::default();
    let mut counter:usize = 0;

    loop{
        parser.0 += 1;
        if parser.0 >= bounds || counter == 8{
            break;
        }
        parser = mapinator(parser.0, bounds, data);
        maps[counter] = parser.1;
        counter += 1;
    }

    Almanac {
        seeds: seeds,
        seed_soil: maps[0].clone(),
        soil_fertilizer: maps[1].clone(),
        fertilizer_water: maps[2].clone(),
        water_light: maps[3].clone(),
        light_temp: maps[4].clone(),
        temp_humidity: maps[5].clone(),
        humidity_locaction: maps[6].clone(),
    }


}
fn seed_separator(index:usize, data: &Vec<String>) -> (usize, Vec<i64>) {
    /*Seed separator, get_seeds, picks up the seeds and separates the stringy 
    bastards into nice and orderly vectors of signed integers. It sticks the 
    vector into a tuple along with the current index..
    
    ..Its always 1, but the farmer is a stickler for indices*/

    let mut seeds: Vec<i64> = Vec::with_capacity(50);
    let seeds_raw: Vec<&str> = data[index].split(":").collect();
    let seeds_as_str: Vec<&str> = seeds_raw[1].split(" ").filter(|&s|!s.is_empty()).collect();
    for el in seeds_as_str{
        //println!("{el}");
        let seed: i64 = el.parse().unwrap();
        seeds.push(seed);
    }
    (1 as usize, seeds)
}
fn mapinator(index: usize, bounds: usize, data: &Vec<String>) -> (usize, Vec<(i64, i64, i64)>){
    /*Mapinator, get_map, takes a reference to the complete data set and a 
    start index. It loops through the set, starting at the given index, 
    and breaks when it encounters an empty string or EOF. Returning the 
    current index in the dataset, along with a vector of tuples, sorted 
    by key 1, representing the complete map*/
    
    let mut map: Vec<(i64, i64, i64)> = Vec::with_capacity(50);
    let mut counter = index as usize;
    loop{
        counter += 1;
        if data[counter] == ""{
            break
        }
        let map_part_as_strings: Vec<&str> = data[counter].split(" ").collect();
        let map_part:(i64, i64, i64) = (
            map_part_as_strings[0].parse().unwrap(),
            map_part_as_strings[1].parse().unwrap(),
            map_part_as_strings[2].parse().unwrap()
        );
        map.push(map_part);
        if counter == bounds - 1{
            break
        }
    }
    map.sort_unstable_by_key(|k| k.1);
    //println!("complete map: {:?}", map);
    (counter, map)
}
fn map_lookup(from:i64, map:&Vec<(i64, i64, i64)>) -> i64{
    let mut to = from;
    for map_part in map{
        if from < map_part.1 || from > map_part.1 + map_part.2{
            continue;
        } else {
            to = from + (map_part.0 - map_part.1);
            break;
        }
    }
    to
}
fn find_lowest_location(data: &Almanac) -> i64 {
    let mut lowest_location: i64 = i64::MAX;
    let maps = [
        &data.seed_soil,
        &data.soil_fertilizer,
        &data.fertilizer_water,
        &data.water_light,
        &data.light_temp,
        &data.temp_humidity,
        &data.humidity_locaction
        ];
    for seed in &data.seeds{
        let mut to = seed.clone();
        for map in maps{
           to = map_lookup(to, map);
        }
        if to < lowest_location{
            lowest_location = to;
        }
    }
    lowest_location
}
fn seed_list_interpreter(data:Vec<i64>){
    let mut index:usize = 0;
}