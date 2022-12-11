use clap::Parser;
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
    #[clap(default_value = "2503")]
    time: u32,
}

struct Raindeer {
    name: String,
    speed: u32,
    stamina: u32,
    rest: u32,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let raindeer: Vec<Raindeer> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_raindeer)
        .collect();

    let total_time = args.time;

    let distances: Vec<u32> = raindeer
        .iter()
        .map(|r| {
            let full_periods = total_time / (r.stamina + r.rest);
            let remainder = total_time % (r.stamina + r.rest);
            let partial_period_distance = if remainder >= r.stamina {
                r.speed * r.stamina
            } else {
                r.speed * remainder
            };

            partial_period_distance + (full_periods * r.stamina * r.speed)
        })
        .collect();

    println!("Winning distance: {}", distances.iter().max().unwrap())
}

fn parse_raindeer(input: String) -> Raindeer {
    let components: Vec<&str> = input.split_whitespace().collect();

    Raindeer {
        name: components[0].to_string(),
        speed: components[3].parse().unwrap(),
        stamina: components[6].parse().unwrap(),
        rest: components[13].parse().unwrap(),
    }
}
