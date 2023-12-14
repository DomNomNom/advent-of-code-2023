#![allow(dead_code)]
use std::{
    cmp::{max, min},
    env,
    fs::{self},
};

// struct ContextIterator<I, T> {
//     iter: I,
//     prev: Option<T>,
//     now: Option<T>,
//     next: Option<T>,
// }
// impl<I, T> ContextIterator<I, T>
// where
//     I: Iterator<Item = T>,
// {
//     fn new(mut iter: I) -> ContextIterator<I, T> {
//         ContextIterator {
//             iter: iter,
//             prev: None,
//             now: None,
//             next: iter.next(),
//         }
//     }
// }
// impl<'a, I: Iterator, T> Iterator for ContextIterator<I, T>
// where
//     I: Iterator<Item = T>,
// {
//     type Item = (Option<&'a mut T>, &'a mut T, Option<&'a mut T>);

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next {
//             None => None,
//             Some(next) => {}
//         }
//     }
// }

// fn map_with_context<T>(it: Iterator<T>, f: FnMut<&'a mut T>)

fn advent03() {
    println!("{:?}", env::current_dir());
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut used: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let mut acc = 0u32;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if used[i][j] || grid[i][j] == '.' || grid[i][j].is_numeric() {
                continue;
            }
            used[i][j] = true;
            // scan around the marker
            for ii in (max(i, 1) - 1)..min(i + 2, grid.len()) {
                for jj in (max(j, 1) - 1)..min(j + 2, grid[i].len()) {
                    if used[ii][jj] {
                        continue;
                    }
                    used[ii][jj] = true;
                    let row = &grid[ii];
                    if row[jj].is_numeric() {
                        // scan left
                        let mut jjj = jj;
                        while jjj > 0 && row[jjj - 1].is_numeric() {
                            jjj -= 1;
                        }
                        // read the value
                        let mut val = 0u32;
                        while jjj < row.len() && row[jjj].is_numeric() {
                            val *= 10;
                            val += row[jjj].to_digit(10).unwrap();
                            used[ii][jjj] = true;
                            jjj += 1;
                        }
                        acc += val;
                    }
                }
            }
        }
    }
    println!("{acc}");
}
fn advent03_part2() {
    println!("{:?}", env::current_dir());
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut used: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let mut acc = 0u32;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if used[i][j] || grid[i][j] == '.' || grid[i][j].is_numeric() {
                continue;
            }
            used[i][j] = true;
            let mut vals: Vec<u32> = vec![];
            // scan around the marker
            for ii in (max(i, 1) - 1)..min(i + 2, grid.len()) {
                for jj in (max(j, 1) - 1)..min(j + 2, grid[i].len()) {
                    if used[ii][jj] {
                        continue;
                    }
                    used[ii][jj] = true;
                    let row = &grid[ii];
                    if row[jj].is_numeric() {
                        // scan left
                        let mut jjj = jj;
                        while jjj > 0 && row[jjj - 1].is_numeric() {
                            jjj -= 1;
                        }
                        // read the value
                        let mut val = 0u32;
                        while jjj < row.len() && row[jjj].is_numeric() {
                            val *= 10;
                            val += row[jjj].to_digit(10).unwrap();
                            used[ii][jjj] = true;
                            jjj += 1;
                        }
                        vals.push(val);
                    }
                }
            }
            if vals.len() == 2 {
                acc += vals.iter().product::<u32>();
            }
        }
    }
    println!("{acc}");
}

fn main() {
    advent03_part2();
}
