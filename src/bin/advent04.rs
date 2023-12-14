#![allow(dead_code)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_int_set(i: &str) -> HashSet<u32> {
    i.split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<HashSet<_>>()
}

fn advent04() {
    let file = File::open("inputs/04.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();
    let mut acc = 0u32;

    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (_, vert) = line.split_once(":").unwrap();
        let (winners, got) = vert.split_once("|").unwrap();
        let (winners2, got2) = (parse_int_set(winners), parse_int_set(got));
        let won_count = winners2.intersection(&got2).count();
        if won_count > 0 {
            acc += 1 << (won_count - 1);
        }
    }
    println!("{acc}");
}

fn advent04_part2() {
    let file = File::open("inputs/04.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    let mut values = vec![];
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (_, vert) = line.split_once(":").unwrap();
        let (winners, got) = vert.split_once("|").unwrap();
        let (winners2, got2) = (parse_int_set(winners), parse_int_set(got));
        // println!(
        //     "{winners2:?} {got2:?} {:?}",
        //     winners2.intersection(&got2).collect::<Vec<&u32>>()
        // );
        values.push(winners2.intersection(&got2).count());
    }
    for i in (0..values.len()).rev() {
        values[i] = 1 + values[(i + 1)..(i + 1 + values[i])].iter().sum::<usize>();
        // println!("{values:?}");
    }
    let acc: usize = values.iter().sum();
    println!("{acc}");
}

fn main() {
    advent04_part2();
}
