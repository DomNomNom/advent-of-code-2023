use itertools::{fold, Itertools};
use std::{cmp::max, f32::consts::PI, fs::read};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}
use Dir::*;

fn step(row: usize, col: usize, dir: &Dir, dist: u8) -> (usize, usize) {
    // this looks nicer on one line per direction but I guess I'll let the formatter do its job
    match dir {
        Dir::N => (row - dist as usize, col),
        Dir::E => (row, col + dist as usize),
        Dir::S => (row + dist as usize, col),
        Dir::W => (row, col - dist as usize),
    }
}

fn main() {
    let input = read("inputs/18.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let instructions = input
        .into_iter()
        .map(|line| {
            let (step, color) = line.split_once(" (#").unwrap();
            let (dir, distance) = step.split_once(" ").unwrap();
            let dir = match dir {
                "U" => N,
                "R" => E,
                "D" => S,
                "L" => W,
                x => unimplemented!("{}", x),
            };
            let distance: u8 = distance.parse().unwrap();
            let color = &color[..6];
            let color: u32 = u32::from_str_radix(color, 16).unwrap();
            (dir, distance, color)
        })
        .collect_vec();
    let mut row = 0;
    let mut col = 0;
    let mut wd = 0usize;
    let mut ht = 0usize;
    for &(dir, dist, _) in instructions.iter() {
        (row, col) = step(row, col, &dir, dist);
        ht = max(ht, row);
        wd = max(wd, col);
    }
    wd += 1;
    ht += 1;
    let mut grid = (0..ht)
        .map(|_| (0..wd).map(|_| 0u32).collect_vec())
        .collect_vec();

    for &(dir, dist, color) in instructions.iter() {
        (row, col) = step(row, col, &dir, dist);
        grid[row][col] = color
    }
    for line in grid {
        for cell in line {
            print!(
                "{}",
                match cell {
                    0 => '.',
                    _ => '#',
                }
            );
        }
        println!();
    }
}
