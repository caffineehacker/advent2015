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

    let weight_third: i64 = weights.iter().sum::<i64>() / 3;

    if args.debug {
        println!("Weight third: {}", weight_third);
    }

    let mut lowest_valid_entanglement = i64::MAX;
    for i in 1..weights.len() {
        if args.debug {
            println!("Checking size of: {}", i);
        }
        for ws in weights
            .iter()
            .combinations(i)
            .filter(|w| w.iter().cloned().sum::<i64>() == weight_third)
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

            for j in 1..weights.len() {
                if weights
                    .iter()
                    .filter(|w| !ws.contains(w))
                    .combinations(j)
                    .any(|lw| lw.iter().cloned().sum::<i64>() == weight_third)
                {
                    lowest_valid_entanglement = quantum_entanglement;
                    break;
                }
            }
        }
        if lowest_valid_entanglement < i64::MAX {
            println!("Part 1: {}", lowest_valid_entanglement);
            break;
        }
    }
}
