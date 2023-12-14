#![allow(dead_code)]
use counter::Counter;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, PartialEq, Eq)]
struct HandBid {
    hand: Vec<u8>, // card ranks mapped onto integers, higher is better.
    bid: u32,
}
fn advent07_hand_type(hand: &Vec<u8>) -> u8 {
    // outputs an ordinal for the hand type. higher is better.
    let c: Counter<&u8, u8> = Counter::from_iter(hand.iter());

    let counts: Vec<u8> = c
        .most_common_ordered()
        .into_iter()
        .map(|(_, count)| count)
        .collect();
    if counts[0] >= 5 {
        10
    } else if counts[0] >= 4 {
        9
    } else if counts[0] >= 3 && counts[1] >= 2 {
        8
    } else if counts[0] >= 3 {
        7
    } else if counts[0] >= 2 && counts[1] >= 2 {
        6
    } else if counts[0] >= 2 {
        5
    } else {
        4
    }
}
fn advent07() {
    let file = File::open("inputs/07.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    let mut hand_bids: Vec<HandBid> = vec![];
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (hand, bid) = line.trim().split_once(" ").unwrap();
        let hand = hand
            .chars()
            .map(|c| {
                "AKQJT98765432"
                    .chars()
                    .rev()
                    .position(|c2| c2 == c)
                    .unwrap() as u8
            })
            .collect();
        hand_bids.push(HandBid {
            hand: hand,
            bid: bid.parse().unwrap(),
        });
    }

    hand_bids.sort_by_key(|hb| {
        let hand_type = advent07_hand_type(&hb.hand);
        (hand_type, hb.hand.clone()) // I wonder how this clone could be avoided.
    });
    //println!("{hand_bids:?}");
    let ans: u32 = hand_bids
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.bid)
        .sum();
    println!("{ans:?}");
}

fn advent07_part2_hand_type(hand: &Vec<u8>) -> u8 {
    // outputs an ordinal for the hand type. higher is better.
    let mut c: Counter<&u8, u8> = Counter::from_iter(hand.iter());
    let joke_count = c.remove(&0u8).unwrap_or(0);

    let mut counts: Vec<u8> = c
        .most_common_ordered()
        .into_iter()
        .map(|(_, count)| count)
        .collect();
    if counts.len() == 0 {
        counts.push(0); // It's all fucking jokers lol
    }
    counts[0] += joke_count;
    if counts[0] >= 5 {
        10
    } else if counts[0] >= 4 {
        9
    } else if counts[0] >= 3 && counts[1] >= 2 {
        8
    } else if counts[0] >= 3 {
        7
    } else if counts[0] >= 2 && counts[1] >= 2 {
        6
    } else if counts[0] >= 2 {
        5
    } else {
        4
    }
}
fn advent07_part2() {
    let file = File::open("inputs/07.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    let mut hand_bids: Vec<HandBid> = vec![];
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (hand, bid) = line.trim().split_once(" ").unwrap();
        let hand = hand
            .chars()
            .map(|c| {
                "AKQJT98765432J"
                    .chars()
                    .rev()
                    .position(|c2| c2 == c)
                    .unwrap() as u8
            })
            .collect();
        hand_bids.push(HandBid {
            hand: hand,
            bid: bid.parse().unwrap(),
        });
    }

    hand_bids.sort_by_key(|hb| {
        let hand_type = advent07_part2_hand_type(&hb.hand);
        (hand_type, hb.hand.clone()) // I wonder how this clone could be avoided.
    });
    //println!("{hand_bids:?}");
    let ans: u32 = hand_bids
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.bid)
        .sum();
    println!("{ans:?}");
}

fn main() {
    advent07();
    advent07_part2();
}
