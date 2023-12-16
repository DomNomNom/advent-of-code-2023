use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read,
};

use itertools::{iproduct, Itertools};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    O,    // rolling ball
    Hash, // fixed cube object
}

fn shift_all(grid: &mut Vec<Vec<Cell>>, move_rows: i16, move_cols: i16) -> bool {
    let mut somechange = false;
    let ht = grid.len();
    let wd = grid[0].len();
    let mut destinations = iproduct!(
        max(0i16, move_rows) as usize..min(ht, (ht as i16 + move_rows) as usize),
        max(0i16, move_cols) as usize..min(wd, (wd as i16 + move_cols) as usize)
    )
    .collect_vec();
    // println!("{:?}", destinations);
    destinations.sort_by_key(|&(row, col)| -(row as i16 * move_rows + col as i16 * move_cols));
    // write to the ones furthest in the shift direction first
    for (dest_row, dest_col) in destinations {
        let src_row = (dest_row as i16 - move_rows) as usize;
        let src_col = (dest_col as i16 - move_cols) as usize;
        if grid[dest_row][dest_col] == Cell::Empty && grid[src_row][src_col] == Cell::O {
            somechange = true;
            grid[dest_row][dest_col] = grid[src_row][src_col];
            grid[src_row][src_col] = Cell::Empty;
        }
    }
    somechange
}

fn calculate_load(grid: &Vec<Vec<Cell>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .map(|&cell| if cell == Cell::O { grid.len() - i } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn print_grid(grid: &Vec<Vec<Cell>>) {
    for line in grid {
        for cell in line {
            print!(
                "{}",
                match cell {
                    Cell::Empty => ".",
                    Cell::O => "O",
                    Cell::Hash => "#",
                }
            );
        }
        println!();
    }
    println!()
}

fn spin_cycle(grid: &mut Vec<Vec<Cell>>) {
    while shift_all(grid, -1, 0) {}
    while shift_all(grid, 0, -1) {}
    while shift_all(grid, 1, 0) {}
    while shift_all(grid, 0, 1) {}
}

fn main() {
    let input = read("inputs/14.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let mut grid = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Hash,
                    'O' => Cell::O,
                    _ => unimplemented!(),
                })
                .collect_vec()
        })
        .collect_vec();

    while shift_all(&mut grid, -1, 0) {}
    let part1_answer = calculate_load(&grid);
    dbg!(part1_answer);

    // note: We're safe to continue with the modified grid from here for part 2 as the spin cycle happens to start with the same direction.

    // for _i in 0..3 {
    //     spin_cycle(&mut grid);
    //     print_grid(&grid);
    // }
    // return;

    let mut seen: HashMap<Vec<Vec<Cell>>, u64> = HashMap::new();
    let mut repeat_start = 0;
    let mut repeat_end = 0;
    for i in 0..10000000u64 {
        println!("{i}");
        if let Some(prev) = seen.insert(grid.clone(), i) {
            repeat_start = prev;
            repeat_end = i;
            break;
        }
        spin_cycle(&mut grid);
    }

    // fast forward cycles until our target
    let target_cycles = 1000000000u64;
    let repeat_len = repeat_end - repeat_start;
    let mut i = repeat_end;
    while i + repeat_len <= target_cycles {
        i += repeat_len; // this could be done quicker with some math. but it's super fast anyway.
    }
    dbg!(i);
    while i < target_cycles {
        dbg!(i);
        i += 1;
        spin_cycle(&mut grid);
    }

    let part2_answer = calculate_load(&grid);
    dbg!(part2_answer); // 83795 is too high
}
