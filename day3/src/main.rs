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
        current_position = execute_move(current_position, instruction);
        visited_houses.insert(current_position);
    }

    println!("Part 1 # of visited houses: {}", visited_houses.len());

    visited_houses.clear();
    visited_houses.insert((0, 0));

    let mut santa_position = (0, 0);
    let mut robo_santa_position = (0, 0);
    let mut is_santas_turn = true;

    for instruction in instructions.chars() {
        let current_position = if is_santas_turn {
            &mut santa_position
        } else {
            &mut robo_santa_position
        };
        *current_position = execute_move(*current_position, instruction);
        visited_houses.insert(*current_position);

        is_santas_turn = !is_santas_turn;
    }

    println!("Part 2 # of visited houses: {}", visited_houses.len());
}

fn execute_move(current_position: (i32, i32), instruction: char) -> (i32, i32) {
    let change = match instruction {
        '^' => (0, -1),
        '>' => (1, 0),
        'v' => (0, 1),
        '<' => (-1, 0),
        '\n' => (0, 0),
        what => panic!("Unexpected input \"{}\"", what),
    };

    (current_position.0 + change.0, current_position.1 + change.1)
}
