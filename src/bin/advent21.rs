use itertools::Itertools;
use std::fs::read;

fn main() {
    let input = read("inputs/21.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
}
