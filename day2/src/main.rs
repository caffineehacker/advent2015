use clap::Parser;
use std::cmp::min;
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

    let total_paper: u32 = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(get_sides)
        .map(get_required_paper)
        .sum();

    println!("Total paper: {}", total_paper);
}

fn get_sides(line: String) -> (u32, u32, u32) {
    let edges: Vec<u32> = line
        .split('x')
        .map(|s| s.parse().expect("Failed to convert string to u32"))
        .collect();

    (
        edges.get(0).unwrap() * edges.get(1).unwrap(),
        edges.get(0).unwrap() * edges.get(2).unwrap(),
        edges.get(1).unwrap() * edges.get(2).unwrap(),
    )
}

fn get_required_paper(sides: (u32, u32, u32)) -> u32 {
    let min_side = min(min(sides.0, sides.1), sides.2);
    min_side + 2 * (sides.0 + sides.1 + sides.2)
}
