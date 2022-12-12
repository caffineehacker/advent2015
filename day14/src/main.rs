use clap::Parser;
use std::{
    collections::HashMap,
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

    let distances: Vec<u32> = get_distances_at_time(&raindeer, total_time)
        .values()
        .cloned()
        .collect();

    println!("Winning distance: {}", distances.iter().max().unwrap());

    let mut overall_score = HashMap::new();
    for raindeer_winner in
        (1..=total_time).flat_map(|t| get_leaders(get_distances_at_time(&raindeer, t)))
    {
        if overall_score.contains_key(raindeer_winner) {
            let raindeer_score = overall_score.get_mut(raindeer_winner).unwrap();
            *raindeer_score += 1;
        } else {
            overall_score.insert(raindeer_winner, 1);
        }
    }

    let (best_raindeer, best_score) = overall_score.iter().max_by_key(|r| *r.1).unwrap();

    println!(
        "Best overall score (part 2): {} = {}",
        best_raindeer, best_score
    );
}

fn get_leaders<'a>(results: HashMap<&'a str, u32>) -> Vec<&'a str> {
    let max_value = *results.iter().max_by_key(|r| r.1).unwrap().1;

    results
        .iter()
        .filter(|r| *r.1 == max_value)
        .map(|r| *r.0)
        .collect()
}

fn get_distances_at_time<'a>(
    raindeer: &'a Vec<Raindeer>,
    total_time: u32,
) -> HashMap<&'a str, u32> {
    raindeer
        .iter()
        .map(|r| {
            let full_periods = total_time / (r.stamina + r.rest);
            let remainder = total_time % (r.stamina + r.rest);
            let partial_period_distance = if remainder >= r.stamina {
                r.speed * r.stamina
            } else {
                r.speed * remainder
            };

            (
                r.name.as_str(),
                partial_period_distance + (full_periods * r.stamina * r.speed),
            )
        })
        .collect()
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
