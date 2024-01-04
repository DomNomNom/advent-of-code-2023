use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Normal;
use std::cmp::min;
use std::collections::VecDeque;
use std::fs::{self, read};
use vec3D::Vec3D;
use yamakan::domains::ContinuousDomain;
use yamakan::optimizers::nelder_mead::NelderMeadOptimizer;

type PosVel = (Vec3D, Vec3D);

type Mat3x3 = [[f64; 3]; 3]; // arrays of rows

fn yaw_pitch_matrix(yaw: f64, pitch: f64) -> Mat3x3 {
    mat_mul(&yaw_matrix(yaw), &pitch_matrix(pitch))
}

// rotates about the vertical (z) axis.
#[rustfmt::skip]
fn yaw_matrix(yaw: f64) -> Mat3x3 {
    [
        [ yaw.cos(), yaw.sin(), 0.0],
        [-yaw.sin(), yaw.cos(), 0.0],
        [       0.0,       0.0, 1.0],
    ]
}

// rotates about the horizontal y axis.
#[rustfmt::skip]
fn pitch_matrix(pitch: f64) -> Mat3x3 {
    [
        [ pitch.cos(), 0.0, pitch.sin()],
        [         0.0, 1.0,         0.0],
        [-pitch.sin(), 0.0, pitch.cos()],
    ]
}
// computes `AB` with matrix multiplication
fn mat_mul(a: &Mat3x3, b: &Mat3x3) -> Mat3x3 {
    let mut out = Mat3x3::default();
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum::<f64>();
        }
    }
    out
}
// Computes `Av` with matrix-vector multiplication
fn mat_vec_mul(a: &Mat3x3, v: &Vec3D) -> Vec3D {
    Vec3D::new(
        a[0][0] * v.x + a[0][1] * v.y + a[0][2] * v.z,
        a[1][0] * v.x + a[1][1] * v.y + a[1][2] * v.z,
        a[2][0] * v.x + a[2][1] * v.y + a[2][2] * v.z,
    )
}

fn transpose(a: &Mat3x3) -> Mat3x3 {
    let mut out = Mat3x3::default();
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = a[j][i];
        }
    }
    out
}
fn vec_to_array(v: &Vec3D) -> [f64; 3] {
    [v.x, v.y, v.z]
}

// adapted from dist3D_Line_to_Line from https://web.archive.org/web/20180704020228/http://www.geomalgorithms.com/a07-_distance.html
fn approach_times(track1: &PosVel, track2: &PosVel) -> (f64, f64) {
    let u = track1.1;
    let v = track2.1;
    let w = track1.0 - track2.0;
    let a = u.dot(u); // always >= 0
    let b = u.dot(v);
    let c = v.dot(v); // always >= 0
    let d = u.dot(w);
    let e = v.dot(w);
    let big_d = a * c - b * b; // always >= 0

    // compute the line parameters of the two closest points
    if (big_d < 0.00000001) {
        println!("parallel");
        // the lines are almost parallel
        (0.0, if b > c { d / b } else { e / c }) // use the largest denominator
    } else {
        ((b * e - c * d) / big_d, (a * e - b * d) / big_d)
    }

    // // get the difference of the two closest points
    // Vector   dP = w + (sc * u) - (tc * v);  // =  L1(sc) - L2(tc)
    // return norm(dP);   // return the closest distance
}

// computes the time when the given two trajectories are closest together.
fn cpa_time(a: &PosVel, b: &PosVel) -> f64 {
    let dv = a.1 - b.1;
    let dv2 = dv.dot(dv);
    if dv2 < 0.0000001 {
        return 0.0; // almost parallel. any time is ok.  Use time 0.
    }
    let w0 = a.0 - b.0;
    -w0.dot(dv) / dv2
}

// computes the distance at the closest approach time
fn cpa_distance(a: &PosVel, b: &PosVel) -> f64 {
    let ctime = cpa_time(a, b);
    let p1 = a.0 + (a.1 * ctime);
    let p2 = b.0 + (b.1 * ctime);
    p1.distance_to(p2) // distance at CPA
}

fn part1(mut particles: Vec<PosVel>) {
    for (ref mut pos, ref mut vel) in particles.iter_mut() {
        pos.z = 0.;
        vel.z = 0.;
    }

    // sanity check - no duplicate particles
    for (i, p1) in particles.iter().enumerate() {
        for (_j, p2) in particles[0..i].iter().enumerate() {
            assert_ne!(p1, p2);
        }
    }
    // sanity check - no still particles
    for (_pos, vel) in particles.iter() {
        assert!(vel.mag() > 0.1);
    }
    for (_pos, vel) in particles.iter() {
        assert!(vel.mag() > 0.1);
    }

    let mut answer1 = 0;
    let test_region_bot = Vec3D::new(7., 7., -100.);
    let test_region_top = Vec3D::new(27., 27., 100.);
    let test_region_bot = Vec3D::new(200000000000000., 200000000000000., -100.);
    let test_region_top = Vec3D::new(400000000000000., 400000000000000., 100.);
    for (i, a) in particles.iter().enumerate() {
        for (_j, b) in particles[0..i].iter().enumerate() {
            if b.1.cross(a.1).mag() < 0.001 {
                // println!("{} {} parallel", i, _j);
                continue; // parallel paths
            }
            let (ta, tb) = approach_times(a, b);

            if ta < 0. || tb < 0. {
                // println!("{} {} in past", i, _j);
                continue;
            }

            let pa = a.0 + a.1 * ta;
            let pb = b.0 + b.1 * tb;
            assert!(pa.distance_to(pb) < 0.00001 * pa.mag());
            // println!("{} {} -> location={} {} at t={} {}", i, _j, pa, pb, ta, tb);
            if false // style
                || pa.x < test_region_bot.x
                || pa.y < test_region_bot.y
                || pa.z < test_region_bot.z
                || pa.x >= test_region_top.x
                || pa.y >= test_region_top.y
                || pa.z >= test_region_top.z
            {
                // println!("{} {} out of box at location={:?}", i, _j, pa);
                continue;
            }
            // println!("{} {} all good", i, _j);
            answer1 += 1;
        }
    }
    dbg!(answer1);
}

fn cross_entropy_method_optimization<F>(
    cost_fn: F,
    mut distribution: Vec<Normal<f64>>,
    mut rng: &mut StdRng,
    sample_size: usize,
    elite_size: usize,
) -> Vec<f64>
where
    F: Fn(&Vec<f64>) -> f64,
{
    // let mut all_samples = vec![];

    loop {
        let sample_size = 1000;
        let elite_size = 20;
        let mut samples = (0..sample_size)
            .map(|_| {
                distribution
                    .iter()
                    .map(|dist| dist.sample(rng))
                    .collect_vec()
            })
            .collect_vec();
        // all_samples.append(&mut samples.clone());
        samples.sort_by_cached_key(|sample| OrderedFloat(cost_fn(sample)));
        // dbg!(samples.iter().map(cost_fn2).collect_vec());

        // Fit a new distribution to the best samples
        let elites = samples[0..elite_size].to_vec();
        println!(
            "cost={} stdev={:?}",
            cost_fn(&elites[elite_size - 1]),
            distribution.iter().map(|d| d.std_dev()).collect_vec()
        );
        let fit = |i| {
            let mean = elites.iter().map(|sample| sample[i]).sum::<f64>() / elite_size as f64;
            let deviation = (elites
                .iter()
                .map(|sample| {
                    let diff = sample[i] - mean;
                    diff * diff
                })
                .sum::<f64>()
                / elite_size as f64)
                .sqrt();
            // let deviation = deviation * 1.4;
            Normal::new(mean, deviation).unwrap()
        };
        distribution = (0..distribution.len()).map(fit).collect();
        // dbg!(distribution.map(|d| d.std_dev()));
        if distribution.iter().all(|d| d.std_dev() < 0.000000000001) {
            return elites[0].clone();
        }
    }
}

fn part2(particles: Vec<PosVel>) {
    let answer2 = 0;

    // let cost_fn_direction = |sample: &Vec<f64>| -> f64 {
    //     if sample[2] < 0. {
    //         return f64::MAX;
    //     }
    //     // a matrix that makes z face the direction of our sample throw direction
    //     let s = Vec3D::new(sample[0], sample[1], sample[2]).norm();
    //     let basis_y = s.cross(Vec3D::new(1., 0., 0.)).norm();
    //     let sample_basis: Mat3x3 = [
    //         vec_to_array(&basis_y),
    //         vec_to_array(&s.cross(basis_y).norm()),
    //         vec_to_array(&s),
    //     ];
    //     let flattened_basis: Mat3x3 = [sample_basis[0], sample_basis[1], [0., 0., 0.]];
    //     // project everything onto the plane to which s is the normal.
    //     let flattened_along_s = particles.iter().map(|p| {
    //         (
    //             mat_vec_mul(&flattened_basis, &p.0),
    //             mat_vec_mul(&flattened_basis, &p.1),
    //         )
    //     });
    //     let intersection_points_xy = flattened_along_s
    //         .tuple_windows::<(_, _)>()
    //         .filter_map(|(a, b)| {
    //             if b.1.cross(a.1).mag() < 0.001 {
    //                 // println!("{} {} parallel", i, _j);
    //                 None
    //             } else {
    //                 let (ta, _tb) = approach_times(&a, &b);
    //                 Some(a.0 + a.1 * ta)
    //             }
    //         })
    //         .collect_vec();
    //     // dbg!(intersection_points_xy.clone());
    //     let mut mean = Vec3D::zeros();
    //     for p in intersection_points_xy.iter() {
    //         mean += *p;
    //     }
    //     mean /= intersection_points_xy.len() as f64;

    //     // dbg!(mean);
    //     // let unflatten = transpose(&sample_basis);
    //     // let unflattened_offset = mat_vec_mul(&unflatten, &mean);
    //     // dbg!(unflattened_offset);

    //     let mut average_distance = 0.;
    //     for p in intersection_points_xy.iter() {
    //         average_distance += mean.distance_to(*p);
    //     }
    //     average_distance /= intersection_points_xy.len() as f64;
    //     average_distance
    // };
    // let mut distribution = (0..3).map(|_| Normal::new(0., 10.).unwrap()).collect_vec();
    // let mut rng = StdRng::seed_from_u64(4);
    // let best_sample =
    //     cross_entropy_method_optimization(cost_fn_direction, distribution, &mut rng, 1000, 20);

    // println!("{best_sample:?}");
    // println!(
    //     "{:?}",
    //     best_sample
    //         .iter()
    //         .map(|x| 6.0 * x / 0.32764794869158237)
    //         .collect_vec()
    // );
    // cost_fn_direction(&vec_to_array(&dir_unscaled).to_vec());
    let dir_unscaled = Vec3D::new(-337.0, -6.0, 155.0);
    println!("{dir_unscaled:?}");
    let unflattened_offset = Vec3D {
        x: 142140533135859.1,
        y: 288472713420856.5,
        z: 320207715789094.56,
    };
    let throw = (unflattened_offset, dir_unscaled);
    let total_dist = particles
        .iter()
        .map(|p| cpa_distance(&throw, p))
        .sum::<f64>();
    println!("{total_dist}");

    let throw_from_sample = |sample: &Vec<f64>| {
        (
            // unflattened_offset + dir_unscaled * sample[0],
            // dir_unscaled * sample[1],
            Vec3D::new(sample[0], sample[1], sample[2]),
            dir_unscaled, // * sample[3],
        )
    };
    let cost_fn_position = |sample: &Vec<f64>| -> f64 {
        let throw = throw_from_sample(sample);
        particles
            .iter()
            .map(|p| cpa_distance(&throw, p))
            .sum::<f64>()
    };
    let mut distribution = vec![
        Normal::new(0., 1000000.).unwrap(),
        Normal::new(0., 1000000.).unwrap(),
        Normal::new(0., 1000000.).unwrap(),
        // Normal::new(10., 100.).unwrap(),
    ];
    let mut rng = StdRng::seed_from_u64(4);
    let best_sample =
        cross_entropy_method_optimization(cost_fn_position, distribution, &mut rng, 1000, 20);
    dbg!(throw_from_sample(&best_sample));

    dbg!(answer2);
}

fn main() {
    let input = read("inputs/24.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let mut particles = input
        .into_iter()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos = pos
                .split(", ")
                .map(|x| x.parse::<f64>().unwrap())
                .collect_vec();
            let vel = vel
                .split(", ")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect_vec();
            (
                Vec3D::new(pos[0], pos[1], pos[2]),
                Vec3D::new(vel[0], vel[1], vel[2]),
            )
        })
        .collect_vec();

    part1(particles.clone());
    part2(particles);
}
