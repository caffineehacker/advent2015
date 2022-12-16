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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let containers: Vec<u32> = reader
        .lines()
        .map(|line| line.expect("Failed to read line").parse().unwrap())
        .collect();

    let eggnog_count = 150;
    let valid_combinations: usize = (1..=containers.len())
        .map(|k| {
            containers
                .iter()
                .combinations(k)
                .filter(|c| c.iter().cloned().sum::<u32>() == eggnog_count)
                .count()
        })
        .sum();

    println!("Valid combinations for 150 liters: {}", valid_combinations);

    let valid_combinations_minimal_count: usize = (1..=containers.len())
        .map(|k| {
            containers
                .iter()
                .combinations(k)
                .filter(|c| c.iter().cloned().sum::<u32>() == eggnog_count)
                .count()
        })
        .filter(|count| *count > 0)
        .nth(0)
        .unwrap();

    println!(
        "Valid minimal combinations for 150 liters: {}",
        valid_combinations_minimal_count
    );
}
