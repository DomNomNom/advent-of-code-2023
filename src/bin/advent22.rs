use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    fs::read,
};

type v3 = [u16; 3];
type aabb = (v3, v3); // inclusive, exclusive bounds

fn intersects_xy(a: &aabb, b: &aabb) -> bool {
    a.0[0] < b.1[0] && b.0[0] < a.1[0] && a.0[1] < b.1[1] && b.0[1] < a.1[1]
}
fn intersects(a: &aabb, b: &aabb) -> bool {
    a.0[0] < b.1[0]
        && b.0[0] < a.1[0]
        && a.0[1] < b.1[1]
        && b.0[1] < a.1[1]
        && a.0[2] < b.1[2]
        && b.0[2] < a.1[2]
}
fn move_to_z(a: &mut aabb, bot_z: u16) {
    a.1[2] = bot_z + (a.1[2] - a.0[2]);
    a.0[2] = bot_z;
}

fn main() {
    let input = read("inputs/22.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let mut input = input
        .into_iter()
        .map(|line| {
            let (l, r) = line.split_once("~").unwrap();
            let parse_v3 = |part: &str| -> v3 {
                part.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            };
            let l = parse_v3(l);
            let mut r = parse_v3(r);
            r[0] += 1;
            r[1] += 1;
            r[2] += 1;
            // let bot = min(l, r);
            // let top = max(l, r);
            assert_eq!(l, min(l, r));
            assert_eq!(r, max(l, r));
            (l, r)
        })
        .collect_vec();

    let to_letter: HashMap<aabb, char> =
        HashMap::from_iter(input.clone().into_iter().enumerate().map(|(i, b)| {
            (
                b,
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().cycle().nth(i).unwrap(),
            )
        }));

    for (j, down) in input.iter().enumerate() {
        for (i, up) in input.iter().enumerate() {
            if up.0[2] < down.0[2] || i == j {
                continue; // wrong direction
            }
        }
    }
    println!();

    // input.sort_by_key(|brick| brick.1[2]); // lowest top-edge first
    input.sort_by_key(|brick| brick.0[2]); // lowest bottom-edge first
    let to_letter: HashMap<usize, char> =
        HashMap::from_iter(input.iter().enumerate().map(|(i, b)| (i, to_letter[b])));

    // make the blocks fall step by step
    // There's a half-working smart algoithm commented out below, so this is a slow, dumb but working one.
    let mut fallen: Vec<aabb> = vec![];
    for (i, mut brick) in input.into_iter().enumerate() {
        while brick.0[2] > 0 && fallen.iter().filter(|b| intersects(&brick, b)).count() == 0 {
            let new_z = brick.0[2] - 1;
            move_to_z(&mut brick, new_z);
        }
        let new_z = brick.0[2] + 1;
        move_to_z(&mut brick, new_z);
        fallen.push(brick);
    }

    // build dependency graph by moving up and down and seeing what we intersect
    let mut to_down: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut to_up: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, mut brick) in fallen.clone().iter_mut().enumerate() {
        let z = brick.0[2];
        move_to_z(&mut brick, z - 1);
        to_down.insert(
            i,
            fallen
                .iter()
                .enumerate()
                .filter_map(|(j, b)| {
                    if i != j && intersects(&brick, b) {
                        Some(j)
                    } else {
                        None
                    }
                })
                .collect_vec(),
        );
        move_to_z(&mut brick, z + 1);
        to_up.insert(
            i,
            fallen
                .iter()
                .enumerate()
                .filter_map(|(j, b)| {
                    if i != j && intersects(&brick, b) {
                        Some(j)
                    } else {
                        None
                    }
                })
                .collect_vec(),
        );
        move_to_z(&mut brick, z);
    }

    // 530
    // for i in 0..fallen.len() {
    //     println!(
    //         "{} -up-> {:?}",
    //         to_letter[&i],
    //         to_up[&i].iter().map(|j| to_letter[j]).collect_vec()
    //     );
    // }

    // for (i, mut up) in input.into_iter().enumerate() {
    //     let mut support_z: Option<u16> = None;
    //     for (j, down) in fallen.iter().enumerate().rev() {
    //         if let Some(z) = support_z {
    //             if down.1[2] > z {
    //                 unreachable!("impossibru");
    //             }
    //             if down.1[2] < z {
    //                 break;
    //             }
    //         }
    //         if intersects_xy(&up, down) {
    //             support_z = Some(down.1[2]);
    //             (*to_down.entry(i).or_default()).push(j);
    //         }
    //     }
    //     if let Some(z) = support_z {
    //         move_to_z(&mut up, z);
    //     } else {
    //         move_to_z(&mut up, 1);
    //         assert_eq!(to_down.insert(i, vec![]), None);
    //     }
    //     fallen.push(up);
    //     fallen.sort_by_key(|brick| brick.1[2]); // lowest top-edge first
    // }

    // for (up, downs) in to_down.iter() {
    //     for down in downs {
    //         (*to_up.entry(*down).or_default()).push(*up);
    //     }
    // }
    // for i in 0..to_down.len() {
    //     if !to_up.contains_key(&i) {
    //         to_up.insert(i, vec![]);
    //     }
    // }

    let answer1 = to_up
        .iter()
        .filter(|&(_k, ups)| ups.iter().all(|up| to_down[up].len() > 1))
        .count();
    // 562 too high
    dbg!(answer1);
    // for brick in bricks {}

    // algorithm:
    // try find paths from the ground to each block without using the zapped block
    let mut fall_count_cache: HashMap<usize, usize> = HashMap::new();
    let grounded_blocks = fallen
        .iter()
        .enumerate()
        .filter_map(|(i, b)| if b.0[2] == 1 { Some(i) } else { None })
        .collect_vec();
    let mut answer2 = 0usize;
    for zapped in 0..fallen.len() {
        let mut seen: HashSet<usize> = HashSet::new();
        let mut q = VecDeque::from(grounded_blocks.clone());
        while let Some(brick) = q.pop_front() {
            if brick == zapped {
                continue;
            }
            if seen.contains(&brick) {
                continue;
            }
            seen.insert(brick);
            for &up in to_up[&brick].iter() {
                q.push_back(up);
            }
        }
        let cascade_count = fallen.len() - seen.len() - 1;
        // println!("cascade from {zapped} = {cascade_count}");
        answer2 += cascade_count;
    }
    dbg!(answer2);
}
