use clap::Parser;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let weights: Vec<i64> = reader
        .lines()
        .map(|line| line.expect("Failed to read line").parse().unwrap())
        .collect();

    let part1 = solve(&args, &weights, 3);
    println!("Part 1: {}", part1);

    let part2 = solve(&args, &weights, 4);
    println!("Part 2: {}", part2);
}

fn solve(args: &Args, weights: &Vec<i64>, compartments: i64) -> i64 {
    let weight_fraction: i64 = weights.iter().sum::<i64>() / compartments;

    if args.debug {
        println!("Weight fraction: {}", weight_fraction);
    }

    let mut lowest_valid_entanglement = i64::MAX;
    for i in 1..weights.len() {
        if args.debug {
            println!("Checking size of: {}", i);
        }
        for ws in weights
            .iter()
            .combinations(i)
            .filter(|w| w.iter().cloned().sum::<i64>() == weight_fraction)
        {
            let quantum_entanglement = ws.iter().fold(1, |acc, w| acc * **w);
            if quantum_entanglement > lowest_valid_entanglement {
                continue;
            }

            if args.debug {
                println!(
                    "Found correct weight with entanglement: {}",
                    quantum_entanglement
                );
            }

            let unused_weights: Vec<&i64> = weights.iter().filter(|w| !ws.contains(w)).collect();
            if any_valid_compartments(unused_weights, compartments - 1, weight_fraction) {
                lowest_valid_entanglement = quantum_entanglement;
                break;
            }
        }
        if lowest_valid_entanglement < i64::MAX {
            return lowest_valid_entanglement;
        }
    }

    -1
}

fn any_valid_compartments(weights: Vec<&i64>, compartments: i64, weight_fraction: i64) -> bool {
    for j in 1..(weights.len() - (compartments as usize - 1)) {
        if weights.iter().combinations(j).any(|lw| {
            if lw.iter().cloned().cloned().sum::<i64>() != weight_fraction {
                return false;
            }

            // If there are only two compartments and one works then the other has to work
            if compartments == 2 {
                return true;
            }

            let weights = weights
                .iter()
                .filter(|w| !lw.contains(w))
                .cloned()
                .collect_vec();
            return any_valid_compartments(weights, compartments - 1, weight_fraction);
        }) {
            return true;
        }
    }

    false
}
