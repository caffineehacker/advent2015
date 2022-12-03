use clap::Parser;
use std::{collections::HashSet, fs};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

fn main() {
    let args = Args::parse();

    let instructions = fs::read_to_string(args.data_file).expect("Failed to read");
    let mut visited_houses = HashSet::new();

    let mut current_position = (0, 0);
    visited_houses.insert(current_position);

    for instruction in instructions.chars() {
        let change = match instruction {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '\n' => (0, 0),
            what => panic!("Unexpected input \"{}\"", what),
        };

        current_position = (current_position.0 + change.0, current_position.1 + change.1);
        visited_houses.insert(current_position);
    }

    println!("# of visited houses: {}", visited_houses.len());
}
