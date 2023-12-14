#![allow(dead_code)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/02.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();
    let mut acc = 0u32;

    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (_game_str, samples) = line.split_once(": ").unwrap();

        // if samples.split("; ").all(|sample| {
        //     sample.trim().split(", ").all(|count_color| {
        //         let (count_str, color) = count_color.split_once(" ").unwrap();
        //         let count: u32 = count_str.parse().unwrap();

        //         if color == "red" {
        //             count <= 12
        //         } else if color == "green" {
        //             count <= 13
        //         } else if color == "blue" {
        //             count <= 14
        //         } else {
        //             panic!("bad color: {color:?}")
        //         }
        //     })
        // }) {
        //     let (_, game_int_str) = game_str.split_once(' ').unwrap();
        //     let game: u32 = game_int_str.parse().unwrap();
        //     acc += game;
        // }

        let count_colors = samples
            .split(';')
            .map(|sample| {
                sample.trim().split(',').map(|count_color| {
                    let (count, color) = count_color.trim().split_once(" ").unwrap();
                    (
                        count.parse::<u32>().unwrap(),
                        ["red", "green", "blue"]
                            .iter()
                            .position(|&c| c == color)
                            .unwrap(),
                    )
                })
            })
            .flatten();
        let mut maxs = [0, 0, 0];
        for (count, color) in count_colors.clone() {
            if count > maxs[color] {
                maxs[color] = count
            }
        }
        // let count_colors = count_colors.collect::<Vec<_>>();
        // println!(
        //     "{samples:?} {count_colors:?} {maxs:?} {}",
        //     maxs.iter().product::<u32>()
        // );
        acc += maxs.iter().product::<u32>();
    }

    print!("{acc}");
}
