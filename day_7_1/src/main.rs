use std::{collections::HashMap, fs::read_to_string};
enum HandValue {
    FiveKind,  // 5
    FourKind,  // 41
    House,     // 32
    ThreeKind, // 311
    TwoPair,   // 221
    Onepair,   // 2111
    HighCard,  // 11111
}

impl HandValue {
    fn value(&self) -> String {
        match *self {
            Self::FiveKind => "5".to_owned(),
            _ => "test".to_owned(),
        }
    }
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
    let five = HandValue::FiveKind.value();
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
                'J' => full_hand.0.push(11),
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
    for (card, amount) in hand_map {
        hand_type_chars.push((amount + 48) as char); //convert int to ASCII-table char
    }
    hand_type_chars.sort();
    hand_type_chars.reverse();
    let hand_type: String = hand_type_chars.iter().collect::<String>();
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
    //[five, four, house, three, twopair, pair, high].into_iter().for_each(|mut vec|{
    //    vec.sort_by_key(|x| x.cards.clone());
    //    vec.reverse();
    //    for el in vec{
    //        sorted_hands.push(el);
    //    }         
    //});
    for mut vec in[five, four, house, three, twopair, pair, high]{
        vec.sort_by_key(|x| x.cards.clone());
        vec.reverse();
        for el in vec{
            sorted_hands.push(el);
        } 
    }

    sorted_hands.reverse();

    sorted_hands
}
