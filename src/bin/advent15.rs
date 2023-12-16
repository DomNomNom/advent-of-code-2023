use itertools::Itertools;
use std::fs::read;

fn hash(s: &str) -> u8 {
    let mut out = 0;
    for c in s.as_bytes() {
        out = c.wrapping_add(out);
        out = out.wrapping_mul(17);
    }
    out
}

#[derive(Default, Debug)]
struct Lens {
    label: String,
    focal: u8,
}

fn main() {
    let input = read("inputs/15.txt").unwrap();

    let input = String::from_utf8(input).unwrap();
    let input = input.trim().split(",").collect_vec();

    // for s in input {
    //     println!("{s:?} {}", hash(s));
    // }
    let answer1 = input.iter().map(|s| hash(s) as u64).sum::<u64>();
    dbg!(answer1);

    let mut boxes: [Vec<Lens>; 256] = [(); 256].map(|_| Vec::new()); // This syntax is unintuitive
    for s in input {
        // println!();
        // println!("{s:?}");
        if &s[s.len() - 1..] == "-" {
            let label = &s[..s.len() - 1];
            let boxx = &mut boxes[hash(label) as usize];
            boxx.retain_mut(|lens| lens.label != label);
        } else {
            let (label, focal) = s.split_once("=").unwrap();
            let focal = focal.parse().unwrap();
            let boxx = &mut boxes[hash(label) as usize];
            let mut found = false;
            for l in boxx.iter_mut() {
                if l.label == label {
                    found = true;
                    l.focal = focal;
                    break;
                }
            }
            if !found {
                boxx.push(Lens {
                    label: label.to_string(),
                    focal,
                });
            }
        }
        // println!("{boxes:?}");
    }

    let answer2 = boxes
        .iter()
        .enumerate()
        .map(|(i, boxx)| {
            (i + 1)
                * boxx
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (j + 1) * lens.focal as usize)
                    .sum::<usize>()
        })
        .sum::<usize>();
    dbg!(answer2);
}
