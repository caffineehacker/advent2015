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
    let floor: i32 = data
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
        .sum();

    println!("Floor: {}", floor);
}
