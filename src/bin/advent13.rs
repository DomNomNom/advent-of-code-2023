use std::{fs::read, iter::zip};

use itertools::Itertools;

fn main() {
    let input = read("inputs/13test.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let input = input
        .into_iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(i, c)| if c == '.' { 0 } else { 1 << i })
                .sum::<u64>()
        })
        .collect_vec();

    for row_above in 0..input.len() - 1 {
        if zip(input[row_above + 1..].iter(), input[..row_above + 1].iter()).all(|(a, b)| a == b) {
            println!("yipiie {row_above}");
        }
    }
    println!("'a'");
}
