use itertools::Itertools;
use std::fs::read;

use memoize::memoize;

#[memoize]
fn solve_12(art: String, nono: Vec<u8>) -> u64 {
    // println!("yay {art:?} {nono:?}");
    if art == "" {
        // println!("yee -> {}", if nono.len() == 0 { 1 } else { 0 });
        return if nono.len() == 0 { 1 } else { 0 };
    }
    let mut rest = &art[1..];
    match &art[0..1] {
        "?" => {
            // println!(
            //     "branch result: {art:?} {nono:?} -> {} {}",
            //     solve_12(format!("#{rest}"), nono.clone()),
            //     solve_12(format!(".{rest}"), nono.clone())
            // );
            let ans =
                solve_12(format!("#{rest}"), nono.clone()) + solve_12(format!(".{rest}"), nono);
            ans
        }
        "." => solve_12(rest.to_owned(), nono),
        "#" => {
            if nono.len() == 0 {
                // println!("nooope A");
                return 0;
            }
            let mut nono = nono.clone();
            let block_len = nono.remove(0);
            assert_ne!(block_len, 0);
            if block_len as usize > art.len()
                    || art[..(block_len as usize)].chars().any(|c| c == '.') // the block must support being all "#"
                    || ((block_len as usize) + 1  < art.len() && &art[block_len as usize..][..1] == "#")
            {
                // println!("nooope B");
                return 0;
            }
            // if block_len as usize <= rest.len() {
            rest = &rest[block_len as usize - 1..]; // take one extra for the '.' if we're not at the end.
                                                    // }
            if rest.len() > 0 {
                if &rest[0..1] == "#" {
                    return 0;
                }
                rest = &rest[1..]; // take one extra for the '.' if we're not at the end.
            }
            solve_12(rest.to_owned(), nono)
        }
        c => unimplemented!("no support for {c:?}"),
    }
}

// fn solve_12_brute_force(art: String, nono: Vec<u8>) -> u64 {
//     let free_bit_count = art.chars().filter(|c| *c == '?').count();
//     (0..(1 << free_bit_count))
//         .filter(|choice| {
//             let mut choice = *choice;
//             let mut completed_art_iter = art.chars().map(|c| {
//                 if c == '?' {
//                     let q = choice & 1;
//                     choice = choice >> 1;
//                     ['#', '.'][q]
//                 } else {
//                     c
//                 }
//             });

//             let mut nono_iter = nono.iter().map(|x| *x);
//             let mut block_remaining: Option<u8> = None; //nono_iter.next();
//             let mut in_block = false;
//             for a in completed_art_iter {
//                 if !in_block {
//                     match a {
//                         '#' => {
//                             assert!(block_remaining.is_none());
//                             block_remaining = nono_iter.next();
//                             if block_remaining.is_none() {
//                                 return false; // completed art has too many blocks
//                             }
//                             in_block = true;
//                         }
//                         '.' => {}
//                         _ => unreachable!(),
//                     }
//                 }
//                 // deliberately not else
//                 if in_block {
//                     match block_remaining {
//                         None => unreachable!(),
//                         Some(0) => {
//                             if a != '.' {
//                                 return false;
//                             }
//                             block_remaining = None;
//                             in_block = false;
//                         }
//                         Some(b) => {
//                             if a != '#' {
//                                 return false;
//                             }
//                             block_remaining = Some(b - 1);
//                         }
//                     }
//                 }
//             }

//             // completed art has too few blocks
//             match block_remaining {
//                 None => {}
//                 Some(0) => {}
//                 _ => return false,
//             };
//             if nono_iter.next().is_some() {
//                 return false;
//             }
//             true
//         })
//         .count() as u64
// }

fn advent12() {
    let input = read("inputs/12.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();

    // combination of ASCII art and 1-D nonogram hint
    let artnonos = input
        .iter()
        .map(|line| {
            let (art, nono) = line.split_once(' ').unwrap();
            let nono: Vec<u8> = nono.split(",").map(|x| x.parse().unwrap()).collect_vec();
            (art, nono)
        })
        .collect_vec();

    let mut answer1: u64 = 0;
    for (art, nono) in artnonos {
        let repeats = 5;
        // if "??#?????##" != art {
        //     continue;
        // }

        let art = std::iter::repeat(art).take(repeats).join("?");
        let nono = std::iter::repeat(nono.iter())
            .take(repeats)
            .flatten()
            .map(|x| *x)
            .collect_vec();

        // let val = solve_12_brute_force(art.to_owned(), nono.clone());
        // // if val != val2 {
        // println!("{art:?} {nono:?} -> {val:?}");
        let val2 = solve_12(art.to_owned(), nono.clone());
        println!("{art:?} {nono:?} -> {val2:?}");
        // }
        answer1 += val2;
        // break;
    }
    dbg!(answer1);
}

fn main() {
    advent12();
}
