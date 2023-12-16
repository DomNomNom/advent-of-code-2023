use std::{fs::read, iter::zip};

use itertools::{iproduct, Itertools};

fn find_row_above_reflection(grid: &Vec<u64>, exclude_row: Option<usize>) -> Option<usize> {
    for row_above in 0..grid.len() - 1 {
        if exclude_row == Some(row_above) {
            continue;
        }
        if zip(
            grid[..row_above + 1].iter().rev(),
            grid[row_above + 1..].iter(),
        )
        .all(|(a, b)| a == b)
        {
            return Some(row_above);
        }
    }
    None
}

fn main() {
    let input = read("inputs/13.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.split("\r\n\r\n");
    let input = input.map(|grid| grid.lines().collect_vec()).collect_vec();
    let widths = input.iter().map(|grid| grid[0].len()).collect_vec();
    let input = input
        .into_iter()
        .map(|grid| {
            grid.into_iter()
                .map(|line| {
                    line.chars()
                        .enumerate()
                        .map(|(i, c)| if c == '.' { 0 } else { 1 << i })
                        .sum::<u64>()
                })
                .collect_vec()
        })
        .collect_vec();

    let mut part1_answer = 0;
    let mut part2_answer = 0;

    for (grid, width) in input.iter().zip(widths) {
        // abandoned fun with bithaxx to do horizontal mirroring
        // for col_left in 0..63 {
        //     if grid.iter().all(|row| {
        //         let mask_high = (u64::MAX >> (col_left + 1)) << (col_left + 1);
        //         let high = row & mask_high;
        //         let low = row - high;
        //         let right = high << (col_left + 1); // right on the ASCII art diagram is higher

        //         // 76543210
        //         // 000000yx
        //         // xy000000
        //         // 000000xy
        //         let left = low.reverse_bits() >> (64 - (col_left + 1));
        //         let compare_count = min(col_left+1, );
        //         left == right
        //     }) {
        //         part1_answer += col_left + 1;
        //     }
        // }

        let row_above = find_row_above_reflection(&grid, None);
        if let Some(row_above) = row_above {
            part1_answer += 100 * (row_above + 1);
        }

        let transposed = (0..width)
            .map(|new_row| {
                grid.iter()
                    .enumerate()
                    .map(|(i, row)| ((row >> new_row) & 1) << i)
                    .sum::<u64>()
            })
            .collect_vec();

        // for row in transposed.iter() {
        //     println!("{row:>64b}");
        // }
        // println!();
        let col_above = find_row_above_reflection(&transposed, None);
        if let Some(col_above) = col_above {
            part1_answer += col_above + 1;
        }

        // part 2
        let mut grid = (*grid).clone();
        let mut found_part2_row = false;
        let mut found_part2_col = false;
        for (row, col) in iproduct!(0..grid.len(), 0..width) {
            grid[row] ^= 1 << col;
            if let Some(row_above2) = find_row_above_reflection(&grid, row_above) {
                part2_answer += 100 * (row_above2 + 1);
                found_part2_row = true;
                break;
            }
            grid[row] ^= 1 << col;
        }

        let mut transposed = transposed.clone();
        for (row, col) in iproduct!(0..width, 0..grid.len()) {
            transposed[row] ^= 1 << col;
            if let Some(col_above2) = find_row_above_reflection(&transposed, col_above) {
                part2_answer += col_above2 + 1;
                found_part2_col = true;
                break;
            }
            transposed[row] ^= 1 << col;
        }

        assert!(found_part2_row ^ found_part2_col);
    }
    dbg!(part1_answer);
    dbg!(part2_answer);
}
