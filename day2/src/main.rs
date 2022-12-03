use clap::Parser;
use std::{
    cmp::{max, min},
    mem::swap,
};
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
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let total_paper: u32 = lines
        .iter()
        .map(get_edges)
        .map(get_sides)
        .map(get_required_paper)
        .sum();

    println!("Total paper: {}", total_paper);

    let total_ribbon: u32 = lines.iter().map(get_edges).map(get_required_ribbon).sum();

    println!("Total ribbon: {}", total_ribbon);
}

fn get_edges(line: &String) -> (u32, u32, u32) {
    let edges: Vec<u32> = line
        .split('x')
        .map(|s| s.parse().expect("Failed to convert string to u32"))
        .collect();

    (
        *edges.get(0).unwrap(),
        *edges.get(1).unwrap(),
        *edges.get(2).unwrap(),
    )
}

fn get_sides(edges: (u32, u32, u32)) -> (u32, u32, u32) {
    (edges.0 * edges.1, edges.0 * edges.2, edges.1 * edges.2)
}

fn get_required_paper(sides: (u32, u32, u32)) -> u32 {
    let min_side = min(min(sides.0, sides.1), sides.2);
    min_side + 2 * (sides.0 + sides.1 + sides.2)
}

fn get_required_ribbon(edges: (u32, u32, u32)) -> u32 {
    let mut min_side = min(edges.0, edges.1);
    let mut second_min_side = max(edges.0, edges.1);

    if edges.2 < second_min_side {
        second_min_side = edges.2;
    }
    if second_min_side < min_side {
        swap(&mut min_side, &mut second_min_side);
    }

    2 * (min_side + second_min_side) + (edges.0 * edges.1 * edges.2)
}
