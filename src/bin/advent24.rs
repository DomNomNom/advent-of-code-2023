use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::prelude::*;
use rand_distr::Normal;
use std::fs::read;
use vec3D::Vec3D;

type PosVel = (Vec3D, Vec3D);

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

fn part2(particles: Vec<PosVel>) {
    let answer2 = 0;
    let cost_fn = |throw: &PosVel| -> f64 {
        particles
            .iter()
            .map(|p| cpa_distance(throw, p))
            .sum::<f64>()
    };
    let cost_fn2 = |sample: &Vec<f64>| -> f64 {
        let mut positions = particles
            .iter()
            .zip(sample.iter())
            .map(|(p, t)| p.0 + p.1 * *t);
        // .collect_vec();
        let first = positions.next().unwrap();
        // let pos_iter = positions
        let mut diffs = positions.map(|pos| (pos - first));
        let first_diff = diffs.next().unwrap();
        diffs.map(|diff| diff.cross(first_diff).mag()).sum() // something cheap that acts like principal component analysis

        // cost_fn(&(
        //     // Vec3D::new(sample[0].floor(), sample[1].floor(), sample[2].floor()),
        //     // Vec3D::new(sample[3].floor(), sample[4].floor(), sample[5].floor()),
        //     // Vec3D::new(sample[0], sample[1], sample[2]),
        //     // Vec3D::new(sample[3], sample[4], sample[5]),
        //     a,
        //     b - a,
        // ))
    };

    // dbg!(cost_fn2(&[24., 13., 10., -3., 1., 2.]));
    // return;

    // let mut samples =

    let mut rng = StdRng::seed_from_u64(4);
    // let mut distribution: [Normal<f64>; 2] = [
    //     // Normal::new(319458320561568., 2676204445423380000.).unwrap(),
    //     // Normal::new(319458320561568., 2676204445423380000.).unwrap(),
    //     // Normal::new(319458320561568., 2676204445423380000.).unwrap(),
    //     // Normal::new(0., 10000.).unwrap(),
    //     // Normal::new(0., 10000.).unwrap(),
    //     // Normal::new(0., 10000.).unwrap(),
    //     // Normal::new(20., 10.).unwrap(),
    //     // Normal::new(10., 10.).unwrap(),
    //     // Normal::new(10., 10.).unwrap(),
    //     // Normal::new(0., 10.).unwrap(),
    //     // Normal::new(0., 10.).unwrap(),
    //     // Normal::new(0., 10.).unwrap(),
    //     Normal::new(0., 1000.).unwrap(),
    //     Normal::new(0., 1000.).unwrap(),
    // ];
    let mut distribution = particles
        .iter()
        .map(|_| Normal::new(100., 100.).unwrap())
        .collect_vec();

    for _ in 0..1000 {
        let sample_size = 200;
        let elite_size = 40;
        let mut samples = (0..sample_size)
            .map(|_| {
                distribution
                    .iter()
                    .map(|dist| dist.sample(&mut rng))
                    .collect_vec()
            })
            .collect_vec();
        samples.sort_by_cached_key(|sample| OrderedFloat(cost_fn2(sample)));
        // dbg!(samples.iter().map(cost_fn2).collect_vec());

        // Fit a new distribution to the best samples
        let elites = samples[0..elite_size].to_vec();
        dbg!(cost_fn2(&elites[elite_size - 1]));
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
        // distribution = [fit(0), fit(1), fit(2), fit(3), fit(4), fit(5)];
        distribution = (0..distribution.len()).map(fit).collect();
        // dbg!(distribution.map(|d| d.std_dev()));
        if distribution.iter().all(|d| d.std_dev() < 0.0001) {
            println!("converged!");
            break;
        }
    }
    dbg!(distribution
        .iter()
        .map(|normal| normal.mean())
        .collect_vec());
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
