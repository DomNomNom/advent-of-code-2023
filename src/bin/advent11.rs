#![allow(dead_code)]
use itertools::Itertools;
use std::{
    cmp::{max, min},
    fs::read,
};

fn advent11() {
    let input = read("inputs/11.txt").unwrap();
    let mut input = input;
    input.retain(|c| *c != b'\r');
    let lines: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .map(|line| line.into())
        .collect();

    let mut galaxies: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter_map(|(i, x)| if *x == b'#' { Some(i as usize) } else { None })
                .collect()
        })
        .collect();
    // dbg!();

    fn expand_vertically(galaxies: &mut Vec<Vec<usize>>) {
        for i in (0..galaxies.len()).rev() {
            if galaxies[i].len() == 0 {
                galaxies.insert(i, vec![]);
            }
        }
    }
    fn transpose(galaxies: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let new_row_count = *galaxies
            .iter()
            .map(|row| row.iter().max().unwrap_or(&0))
            .max()
            .unwrap()
            + 1;
        let mut out = (0..new_row_count).map(|_| vec![]).collect_vec();
        for (i, row) in galaxies.iter().enumerate() {
            for &j in row.iter() {
                out[j as usize].push(i);
            }
        }
        for row in out.iter_mut() {
            row.sort();
        }
        out
    }

    expand_vertically(&mut galaxies);
    galaxies = transpose(galaxies);
    expand_vertically(&mut galaxies);
    galaxies = transpose(galaxies);
    // for row in galaxies.iter() {
    //     println!("{row:?}");
    // }

    let coords = galaxies
        .iter()
        .enumerate()
        .map(|(row, cols)| cols.iter().map(|&col| (row, col)).collect_vec())
        .flatten()
        .collect_vec();
    // dbg!(coords);
    let mut answer_part1 = 0;
    fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
        max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
    }
    for (i, a) in coords.iter().enumerate() {
        for b in coords[i + 1..].iter() {
            answer_part1 += distance(a, b);
        }
    }
    dbg!(answer_part1);
}

fn advent11_part2() {
    let input = read("inputs/11.txt").unwrap();
    let mut input = input;
    input.retain(|c| *c != b'\r');
    let lines: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .map(|line| line.into())
        .collect();

    let galaxies: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter_map(|(i, x)| if *x == b'#' { Some(i as usize) } else { None })
                .collect()
        })
        .collect();

    let mut coords = galaxies
        .iter()
        .enumerate()
        .map(|(row, cols)| cols.iter().map(|&col| (row, col)).collect_vec())
        .flatten()
        .collect_vec();

    fn expand_rows(coords: &mut Vec<(usize, usize)>) {
        let mut total_new_space = 0usize;
        coords.sort();
        let expansion_factor = 1000000 - 1usize;
        let max_row = coords.iter().map(|(a, _b)| *a).max().unwrap();
        let mut space_iter = 0..=max_row + 1;
        let mut coo_iter = coords.iter_mut();
        let mut row = space_iter.next().unwrap();
        let mut coo = coo_iter.next().unwrap();

        // would be neat if something like this was possible:
        // while let (Some(row2), Some(coo2)) = (row, coo) {
        loop {
            if row < coo.0 {
                total_new_space += expansion_factor;
                row = space_iter.next().unwrap();
            } else if row == coo.0 {
                row = space_iter.next().unwrap();
            } else {
                coo.0 += total_new_space;
                if let Some(coo2) = coo_iter.next() {
                    coo = coo2;
                } else {
                    break;
                }
            }
        }
    }

    fn transpose(coords: &mut Vec<(usize, usize)>) {
        for (a, b) in coords.iter_mut() {
            std::mem::swap(a, b); //(a, b) = (b, a);
        }
        coords.sort();
    }

    expand_rows(&mut coords);
    transpose(&mut coords);
    expand_rows(&mut coords);
    transpose(&mut coords);
    // println!("{coords:?}");

    let mut answer_part2 = 0;
    fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
        max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
    }
    for (i, a) in coords.iter().enumerate() {
        for b in coords[i + 1..].iter() {
            answer_part2 += distance(a, b);
        }
    }
    dbg!(answer_part2);
}

fn main() {
    advent11();
    advent11_part2();
}
