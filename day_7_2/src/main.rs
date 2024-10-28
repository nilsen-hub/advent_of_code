use std::{collections::HashMap, fs::read_to_string};
enum HandValue {
    FiveKind,  // 5 no
    FourKind,  // 41 make 5
    House,     // 32 make 5
    ThreeKind, // 311 make 41
    TwoPair,   // 221 make 32 or 41
    Onepair,   // 2111 make 311
    HighCard,  // 11111 make 2111
}
#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u32>,
    bid: u32,
    value: String,
}

fn main() {
    let path = "./data/day7";
    let full_data = get_list_from_file(path);
    let parsed = parser(&full_data);
    let mut hands: Vec<Hand> = Vec::with_capacity(1000);
    let mut value_accumulator: u32 = 0;
    for el in parsed {
        let out = build_hand(el);
        hands.push(out);
    }
    let first = sorting_1st(hands);
    for (index, el) in first.iter().enumerate() {
        value_accumulator += el.bid * (index + 1) as u32;
    }
    println!("{}", value_accumulator);
    for el in first {
        println!("{:?}, {}, {}", el.cards, el.bid, el.value);
    }
}
fn get_list_from_file(path: &str) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn parser(full_data: &Vec<String>) -> Vec<(Vec<u32>, u32)> {
    /* returns vector of tuples, t.0 represents card values, t.1 represents
    bid, parsed data is meant to be used with the build_hand struct
    constructor*/
    let mut output: Vec<(Vec<u32>, u32)> = Vec::with_capacity(1000);
    for line in full_data {
        let mut full_hand: (Vec<u32>, u32) = (Vec::with_capacity(5), 0);
        let split: (&str, &str) = line.split_once(" ").unwrap();
        full_hand.1 = split.1.parse().unwrap();
        for c in split.0.chars() {
            if c.is_numeric() {
                full_hand.0.push(c as u32 - 48); //ASCII table to int conversion
                continue;
            }
            match c {
                'A' => full_hand.0.push(14),
                'K' => full_hand.0.push(13),
                'Q' => full_hand.0.push(12),
                'J' => full_hand.0.push(1),
                'T' => full_hand.0.push(10),
                _ => println!("SOMETHING HORRIBLE HAS HAPPENED!!!"),
            }
        }
        output.push(full_hand);
    }
    output
}
fn card_analyzer(cards: &Vec<u32>) -> String {
    
    let mut hand_map: HashMap<u32, u8> = HashMap::with_capacity(5);
    let mut hand_type_chars: Vec<char> = Vec::with_capacity(5);
    for el in cards {
        let count = hand_map.entry(*el).or_insert(0);
        *count += 1;
    }

    for (card, amount) in &hand_map {
        hand_type_chars.push((amount + 48) as char); //convert int to ASCII-table char
    }
    hand_type_chars.sort();
    hand_type_chars.reverse();
    let mut hand_type: String = hand_type_chars.iter().collect::<String>();
    if hand_map.contains_key(&1) {
        if hand_type == "221" {
            if hand_map.get(&1).copied().unwrap_or(0) == 1 {
                hand_type = String::from("32");
            } else {
                hand_type = String::from("41");
            }
        } else {
            match &hand_type as &str {
                "41" => hand_type = String::from("5"),
                "32" => hand_type = String::from("5"),
                "311" => hand_type = String::from("41"),
                "2111" => hand_type = String::from("311"),
                "11111" => hand_type = String::from("2111"),
                "5" => hand_type = String::from("5"),
                _ => println!("something is wrong in the analyzer"),
            }
        }
    }
    hand_type
}
fn build_hand(data: (Vec<u32>, u32)) -> Hand {
    let value = card_analyzer(&data.0);
    Hand {
        cards: data.0,
        bid: data.1,
        value: value,
    }
}
fn sorting_1st(data: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_hands: Vec<Hand> = Vec::new();
    let mut five: Vec<Hand> = Vec::new();
    let mut four: Vec<Hand> = Vec::new();
    let mut house: Vec<Hand> = Vec::new();
    let mut three: Vec<Hand> = Vec::new();
    let mut twopair: Vec<Hand> = Vec::new();
    let mut pair: Vec<Hand> = Vec::new();
    let mut high: Vec<Hand> = Vec::new();

    for hand in data {
        match hand.value.as_str() {
            "5" => five.push(hand),
            "41" => four.push(hand),
            "32" => house.push(hand),
            "311" => three.push(hand),
            "221" => twopair.push(hand),
            "2111" => pair.push(hand),
            "11111" => high.push(hand),
            _ => println!("Something went wrong in sorting_1st"),
        }
    }

    five.sort_by_key(|x| x.cards.clone());
    four.sort_by_key(|x| x.cards.clone());
    house.sort_by_key(|x| x.cards.clone());
    three.sort_by_key(|x| x.cards.clone());
    twopair.sort_by_key(|x| x.cards.clone());
    pair.sort_by_key(|x| x.cards.clone());
    high.sort_by_key(|x| x.cards.clone());

    five.reverse();
    four.reverse();
    house.reverse();
    three.reverse();
    twopair.reverse();
    pair.reverse();
    high.reverse();

    for el in five {
        sorted_hands.push(el);
    }
    for el in four {
        sorted_hands.push(el);
    }
    for el in house {
        sorted_hands.push(el);
    }
    for el in three {
        sorted_hands.push(el);
    }
    for el in twopair {
        sorted_hands.push(el);
    }
    for el in pair {
        sorted_hands.push(el);
    }
    for el in high {
        sorted_hands.push(el);
    }

    sorted_hands.reverse();

    sorted_hands
}
