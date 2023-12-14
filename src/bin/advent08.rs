#![allow(dead_code)]
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

fn advent08() {
    let file = File::open("inputs/08.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    buf_reader.read_line(&mut line).unwrap();
    let instructions: Vec<usize> = line.trim().chars().map(|c| "LR".find(c).unwrap()).collect();
    line.clear();
    buf_reader.read_line(&mut line).unwrap();

    let mut branches: HashMap<&str, [&str; 2]> = HashMap::new();
    let r = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    let mut rest: String = "".to_string();
    let _ = buf_reader.read_to_string(&mut rest);
    for (_, [src, l, r]) in r.captures_iter(rest.as_str()).map(|c| c.extract()) {
        branches.insert(src, [l, r]);
    }

    let mut loc = "AAA";
    for i in 0..999999999 {
        if loc == "ZZZ" {
            println!("{i}");
            break;
        }
        loc = branches.get(loc).unwrap()[instructions[i % instructions.len()]];
    }
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}
fn advent08_part2() {
    let file = File::open("inputs/08.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    buf_reader.read_line(&mut line).unwrap();
    let instructions: Vec<usize> = line.trim().chars().map(|c| "LR".find(c).unwrap()).collect();
    line.clear();
    buf_reader.read_line(&mut line).unwrap();

    let mut branches: HashMap<&str, [&str; 2]> = HashMap::new();
    let r = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    let mut rest: String = "".to_string();
    let _ = buf_reader.read_to_string(&mut rest);
    for (_, [src, l, r]) in r.captures_iter(rest.as_str()).map(|c| c.extract()) {
        branches.insert(src, [l, r]);
    }

    // confirm that we end up in cycle land.
    // let mut fringe: Vec<&str> = branches
    //     .keys()
    //     .map(|k| *k)
    //     .filter(|k| k.chars().nth(2).unwrap() == 'A')
    //     .collect();
    // for i in 0..99999999999 {
    //     if i % 1000000 == 0 {
    //         dbg!(i);
    //     }
    //     if fringe.iter().all(|f| f.chars().nth(2).unwrap() == 'Z') {
    //         println!("{i}");
    //         break;
    //     }
    //     let instruction = instructions[i % instructions.len()];
    //     fringe = fringe
    //         .iter()
    //         .map(|loc| branches.get(loc).unwrap()[instruction])
    //         .collect();
    // }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Cycle {
        len: u64,
        z_times: Vec<u64>, // z_times.all(|t| we're at a 'Z' location at n*len+t)
    }
    let starts: Vec<&str> = branches
        .keys()
        .map(|k| *k)
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .collect();
    // let starts = vec!["AAA"];
    let cycles: Vec<Cycle> = starts
        .iter()
        .map(|s| {
            let mut visited: HashMap<_, u64> = HashMap::new();
            let mut finishes: Vec<u64> = vec![];
            let mut loc = *s;
            for i in 0..(branches.len() * instructions.len()) as u64 {
                let instruction_index = i as usize % instructions.len();
                if let Some(prev_i) = visited.insert((instruction_index, loc), i) {
                    dbg!(prev_i);
                    return Cycle {
                        len: i - prev_i,
                        z_times: finishes.into_iter().filter(|f| *f > prev_i).collect(),
                    };
                }

                loc = branches.get(loc).unwrap()[instructions[instruction_index]];
                if loc.chars().nth(2).unwrap() == 'Z' {
                    finishes.push(i);
                }
            }
            unreachable!();
        })
        .collect();
    // dbg!(starts);
    assert!(cycles.iter().all(|c| c.z_times.len() == 1));
    assert!(cycles.iter().all(|c| c.z_times[0] == c.len - 1));
    let ans = cycles.iter().map(|c| c.len).reduce(lcm).unwrap(); // - 1;

    dbg!(ans);
    // azazaz
    // aazaaz
    dbg!(cycles);
}

fn main() {
    advent08();
    advent08_part2();
}
