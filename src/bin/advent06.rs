#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn advent06() {
    let file = File::open("inputs/06.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, times) = line.split_once(":").unwrap();
    let times: Vec<u64> = times
        .split(" ")
        .map(|t| t.trim())
        .filter(|t| t.len() > 0)
        .map(|t| t.parse().unwrap())
        .collect();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, distances) = line.split_once(":").unwrap();
    let distances: Vec<u64> = distances
        .split(" ")
        .map(|t| t.trim())
        .filter(|t| t.len() > 0)
        .map(|t| t.parse().unwrap())
        .collect();

    let mut acc = 1;
    for (race_duration, record) in zip(times, distances) {
        let win_possibilities: u64 = (0..race_duration)
            .filter(|button_duration| {
                let distance = (race_duration - button_duration) * button_duration;
                distance > record
            })
            .map(|_| 1)
            .sum();
        acc *= win_possibilities;
    }
    println!("{acc:?}");
}

fn advent06_part2() {
    let file = File::open("inputs/06.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, race_duration) = line.split_once(":").unwrap();
    let race_duration: u64 = race_duration
        .split(" ")
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, record) = line.split_once(":").unwrap();
    let record: u64 = record
        .split(" ")
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    // This could be optimized by solving a quadratic equation but my input isn't big enough.
    let mut acc = 1;
    let win_possibilities: u64 = (0..race_duration)
        .filter(|button_duration| {
            let distance = (race_duration - button_duration) * button_duration;
            distance > record
        })
        .map(|_| 1)
        .sum();
    acc *= win_possibilities;
    println!("{acc:?}");
}

fn main() {
    advent06();
    advent06_part2();
}
