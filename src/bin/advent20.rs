use chumsky::{prelude::*, text::ident};
use itertools::Itertools;
use std::{
    collections::{hash_map, HashMap, HashSet, VecDeque},
    fs::{self, read},
    process::Output,
};

type OutWiring = Vec<(String, usize)>; // the usize is the index into Nand input cache

#[derive(Clone, Debug)]
struct FlipFlop {
    state: bool, // true => ON
}
#[derive(Clone, Debug)]
struct Nand {
    input_cache: Vec<bool>, // true => HIGH
}
// #[derive(Clone, Debug)]
// struct Passthrough {}

#[derive(Clone, Debug)]
enum Gate {
    Nand(Nand),
    FlipFlop(FlipFlop),
    Passthrough,
}
type GatesAndNetwork = (HashMap<String, Gate>, HashMap<String, OutWiring>);

fn parser() -> impl Parser<char, GatesAndNetwork, Error = Simple<char>> {
    let wires = just(" -> ")
        .ignore_then(ident().separated_by(just(", ")))
        .map(|names| names.into_iter().map(|name| (name, 0usize)).collect_vec());

    let gate1 =
        just("%")
            .ignore_then(ident())
            .then(wires)
            .map(|(name, wires): (String, OutWiring)| {
                (name, Gate::FlipFlop(FlipFlop { state: false }), wires)
            });
    let gate2 = just("&")
        .ignore_then(ident())
        .then(wires)
        .map(|(name, wires)| {
            (
                name,
                Gate::Nand(Nand {
                    input_cache: vec![],
                }),
                wires,
            )
        });
    let gate3 = ident()
        .then(wires)
        .map(|(name, wires)| (name, Gate::Passthrough, wires));
    let line = gate1.or(gate2).or(gate3);
    let lines = line.padded().repeated();
    lines.map(|lines| {
        let mut gates: HashMap<String, Gate> = HashMap::new();
        let mut network: HashMap<String, OutWiring> = HashMap::new();
        let mut keys: Vec<String> = Vec::new();
        for (name, gate, wires) in lines {
            gates.insert(name.to_string(), gate);
            network.insert(name.to_string(), wires);
            keys.push(name);
        }

        // Creates dangling "output" nodes
        let mut new_nodes = vec![];
        for (_src, targets) in network.iter() {
            for (target_name, _) in targets {
                if !gates.contains_key(target_name) {
                    // assert_eq!(target_name, "output"); // This should be special
                    // println!("creating dangling output node: {target_name:?}");
                    new_nodes.push(target_name.to_string());
                }
            }
        }
        for name in new_nodes {
            gates.insert(name.to_string(), Gate::Passthrough);
            network.insert(name.to_string(), vec![]);
        }

        // initialize the wiring and NAND input caches
        for name in keys {
            for (target_name, index) in network.get_mut(&name).unwrap().iter_mut() {
                *index = match &mut gates.get_mut(target_name).unwrap() {
                    Gate::Nand(nand) => {
                        nand.input_cache.push(false);
                        nand.input_cache.len() - 1usize
                    }
                    _ => 0usize,
                }
            }
        }

        return (gates, network);
    })
}

fn pulse(
    gates: &mut HashMap<String, Gate>,
    network: &HashMap<String, OutWiring>,
    pulse_start_location: &(String, usize),
    high: bool,
) -> (u64, u64) {
    let mut q = VecDeque::from(vec![(pulse_start_location, high)]);
    let mut high_count = 0u64;
    let mut low_count = 0u64;
    while let Some(((name, input_index), high)) = q.pop_front() {
        // apply state change
        let high = match &mut gates.get_mut(name).unwrap() {
            Gate::Nand(nand) => {
                nand.input_cache[*input_index] = high;
                // println!("nand update! {:?} but was {}", nand.input_cache, high);
                !nand.input_cache.iter().all(|state| *state)
            }
            Gate::FlipFlop(ff) => {
                if high {
                    continue;
                } else {
                    ff.state = !ff.state;
                    ff.state
                }
            }
            Gate::Passthrough => high,
        };

        let foo = &network[name];
        if high {
            high_count += foo.len() as u64;
        } else {
            low_count += foo.len() as u64;
        }
        for target in foo {
            // println!(
            //     "{name} -{}-> {}",
            //     if high { "high" } else { "low" },
            //     target.0,
            //     // target.1
            // );

            q.push_front((target, high));
        }
    }
    // dbg!(low_count);
    // dbg!(high_count);
    (low_count, high_count)
}

fn pulse_and_listen_for_low_input(
    gates: &mut HashMap<String, Gate>,
    network: &HashMap<String, OutWiring>,
    pulse_start_location: &(String, usize),
    high: bool,
    listener: &str,
) -> bool {
    let mut q = VecDeque::from(vec![(pulse_start_location, high)]);
    let mut high_count = 0u64;
    let mut low_count = 0u64;
    let mut out = false;
    while let Some(((name, input_index), high)) = q.pop_front() {
        // listen for our end condition
        if name == listener && !high {
            out = true; // not returning just yet so we can apply the full state update
        }

        // apply state change
        let high = match &mut gates.get_mut(name).unwrap() {
            Gate::Nand(nand) => {
                nand.input_cache[*input_index] = high;
                // println!("nand update! {:?} but was {}", nand.input_cache, high);
                !nand.input_cache.iter().all(|state| *state)
            }
            Gate::FlipFlop(ff) => {
                if high {
                    continue;
                } else {
                    ff.state = !ff.state;
                    ff.state
                }
            }
            Gate::Passthrough => high,
        };

        let foo = &network[name];
        for target in foo {
            // println!(
            //     "{name} -{}-> {}",
            //     if high { "high" } else { "low" },
            //     target.0,
            //     // target.1
            // );

            q.push_front((target, high));
        }
    }
    out
}

fn separate_graph(gw: GatesAndNetwork, ends: Vec<&str>) -> (GatesAndNetwork, GatesAndNetwork) {
    let (mut gates, mut network) = gw;
    let mut gates2: HashMap<String, Gate> = HashMap::new();
    let mut network2: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut q = VecDeque::from(ends.clone());
    let mut seen = HashSet::new();
    while let Some(k) = q.pop_front() {
        if !seen.insert(k.to_string()) {
            continue;
        }

        // find all nodes pointing to this one in O(n) time.
        // overall, O(n^2) is fast enough for our input.
        for (prev, wires) in network.iter() {
            for (target, _) in wires {
                if target == k {
                    q.push_back(prev);
                }
            }
        }
    }

    for k in seen.iter() {
        let k = k.as_str();
        if k == "broadcaster" || k == "button" || ends.contains(&k) {
            // handle nodes that appear in both graphs
            gates2.insert(k.to_string(), gates.get(k).unwrap().clone());
            network2.insert(
                k.to_string(),
                network
                    .get(k)
                    .unwrap()
                    .clone()
                    .into_iter()
                    .filter(|(target, _index)| {
                        seen.contains(target.as_str()) || target == "broadcaster"
                    })
                    .collect_vec(),
            );
            network.insert(
                k.to_string(),
                network
                    .get(k)
                    .unwrap()
                    .clone()
                    .into_iter()
                    .filter(|(target, _index)| {
                        !seen.contains(target.as_str()) || target == "broadcaster"
                    })
                    .collect_vec(),
            );
        } else {
            // These middle nodes get transferred to graph 2.
            gates2.insert(k.to_string(), gates.remove(k).unwrap());
            network2.insert(k.to_string(), network.remove(k).unwrap());
        }
    }

    ((gates, network), (gates2, network2))
}

fn save_graph(gates: &HashMap<String, Gate>, wires: &HashMap<String, OutWiring>, path: &str) {
    let mut out = String::new();
    let format_node_name = |name: &str| {
        format!(
            "{}{name}",
            match gates.get(name) {
                Some(Gate::Nand(_)) => "&",
                _ => "",
            }
        )
    };
    for (source, targets) in wires.iter() {
        for (target, i) in targets.iter() {
            out += format!(
                "{} {}{}\r\n",
                format_node_name(source),
                format_node_name(target),
                if *i > 0 {
                    format!(" {i}")
                } else {
                    "".to_string()
                }
            )
            .as_str();
        }
    }
    out = out.lines().sorted().join("\r\n");
    let _ = fs::write(path, out);
}

fn pulses_until_target_receives_low((gates, network): &mut GatesAndNetwork, target: &str) -> u64 {
    let mut count = 0;
    // let (mut& gates, mut& network) = gw;
    loop {
        count += 1;
        let got_low = pulse_and_listen_for_low_input(
            gates,
            network,
            &("button".to_string(), 0),
            false,
            target,
        );
        if got_low {
            return count;
        }
        if count % 100000 == 0 {
            dbg!(count);
        }
    }
}
pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}

fn main() {
    let input = read("inputs/20.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    // let input = input.lines().collect_vec();
    let (mut gates, mut network) = parser().parse(input).unwrap();

    gates.insert("button".to_string(), Gate::Passthrough);
    network.insert("button".to_string(), vec![("broadcaster".to_string(), 0)]);

    let original_gates = gates.clone();

    // dbg!(gates);
    // dbg!(network);
    let mut low_count = 0u64;
    let mut high_count = 0u64;
    for _ in 0..1000 {
        let (a, b) = pulse(&mut gates, &network, &("button".to_string(), 0), false);
        low_count += a;
        high_count += b;
    }
    let answer1 = low_count * high_count;
    // dbg!(low_count);
    // dbg!(high_count);
    // dbg!(answer1);

    // part2
    // Theres multiple distinct Conjunctions leading up to the final conjuction.
    // Solve those independently.
    gates = original_gates;
    save_graph(&gates, &network, "outputs/20_gw.graph.txt");
    let gw = (gates, network);
    let (mut gw1, mut gw2) = separate_graph(gw, vec!["ls"]);
    let (mut gw1, mut gw3) = separate_graph(gw1, vec!["nb"]);
    let (mut gw1, mut gw4) = separate_graph(gw1, vec!["vc"]);
    let (mut gw1, mut gw5) = separate_graph(gw1, vec!["vg"]);
    save_graph(&gw1.0, &gw1.1, "outputs/20_gw1.graph.txt");
    save_graph(&gw2.0, &gw2.1, "outputs/20_gw2.graph.txt");
    save_graph(&gw3.0, &gw3.1, "outputs/20_gw3.graph.txt");
    save_graph(&gw4.0, &gw4.1, "outputs/20_gw4.graph.txt");
    save_graph(&gw5.0, &gw5.1, "outputs/20_gw5.graph.txt");

    // for (mut gw, listener) in [(gw2, "ls"), (gw3, "nb"), (gw4, "vc"), (gw5, "vg")] {
    //     let a = pulses_until_target_receives_low(&mut gw, listener);
    //     let b = pulses_until_target_receives_low(&mut gw, listener);
    //     let c = pulses_until_target_receives_low(&mut gw, listener);
    //     let d = pulses_until_target_receives_low(&mut gw, listener);
    //     dbg!(a);
    //     dbg!(b);
    //     dbg!(c);
    //     dbg!(d);
    //     println!();
    // }

    let big_cycle_len = [(gw3, "nb"), (gw4, "vc"), (gw5, "vg")]
        .iter_mut()
        .map(|(ref mut gw, listener)| pulses_until_target_receives_low(gw, listener))
        .reduce(lcm)
        .unwrap();
    dbg!(big_cycle_len);
    let mut answer2 = pulses_until_target_receives_low(&mut gw2, "ls");
    let increment = pulses_until_target_receives_low(&mut gw2, "ls");
    assert_eq!(increment, pulses_until_target_receives_low(&mut gw2, "ls"));
    assert_ne!(answer2, increment);
    // haha, head empty. just CPU go brr.
    while answer2 % big_cycle_len != 0 {
        answer2 += increment;
    }
    dbg!(answer2);

    // Now figure out when  then figure out

    return;
    // if !gates.contains_key("rx") || true {
    //     // println!("not doing part2");
    //     return;
    // }
    // gates.insert(
    //     "rx".to_string(),
    //     Gate::Nand(Nand {
    //         input_cache: vec![true],
    //     }),
    // );
    // let mut answer2 = 0u64;
    // while match gates.get("rx").unwrap() {
    //     Gate::Nand(nand) => nand.input_cache[0],
    //     _ => true,
    // } {
    //     let _ = pulse(&mut gates, &network, &("button".to_string(), 0), false);
    //     answer2 += 1;
    //     if answer2 % 10000 == 0 {
    //         dbg!(answer2);
    //     }
    // }
    // dbg!(answer2);
}
