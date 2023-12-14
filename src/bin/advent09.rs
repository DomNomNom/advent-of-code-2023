#![allow(dead_code)]
use itertools::Itertools;
use std::fs::{self};

fn parse_int_list(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn advent09() {
    let input = fs::read("inputs/09.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.trim();
    let lines: std::str::Split<'_, &str> = input.split("\n");
    let mut sequences: Vec<Vec<i64>> = lines.map(parse_int_list).collect();

    // This is for part2. wasn't worth to copy paste the code.
    // Comment it out if you want part1.
    for s in &mut sequences {
        s.reverse();
    }

    fn extrapolate(seq: &Vec<i64>) -> i64 {
        if seq.iter().all(|x| *x == 0) {
            return 0;
        }
        let diffs = &seq
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| b - a)
            .collect();
        seq[seq.len() - 1] + extrapolate(diffs)
    }
    let ans: i64 = sequences.iter().map(extrapolate).sum();
    dbg!(ans);
}

fn main() {
    advent09();
}
