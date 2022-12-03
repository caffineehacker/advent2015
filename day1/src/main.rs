use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

fn main() {
    let args = Args::parse();

    let data = fs::read_to_string(args.data_file).expect("Failed to read file");
    let transitions: Vec<i32> = data
        .chars()
        .map(|c| {
            if c == '(' {
                1
            } else if c == ')' {
                -1
            } else {
                0
            }
        })
        .collect();

    println!("Floor: {}", transitions.iter().sum::<i32>());

    // Part 2
    let mut floor = 0;
    for (pos, transition) in transitions.iter().enumerate() {
        floor += transition;
        if floor < 0 {
            println!("Entered basement on {} character", pos + 1);
            break;
        }
    }
}
