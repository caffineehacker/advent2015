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
}

#[derive(PartialEq, Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle,
}

struct LightRun {
    action: Action,
    start: (u32, u32),
    end: (u32, u32),
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    // To make this efficient, we're going to use a kind of run length encoding
    let runs: Vec<LightRun> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .map(input_to_light_run)
        .collect();

    let lights_on = (0..=999)
        .flat_map(|x: u32| {
            let x = x.clone();
            let runs = &runs;
            (0..=999).map(move |y: u32| is_light_on(runs, (x, y)))
        })
        .filter(|l| *l)
        .count();

    println!("Number of lights on: {}", lights_on);
}

fn is_light_on(runs: &Vec<LightRun>, light: (u32, u32)) -> bool {
    runs.iter()
        .filter(|r| {
            r.start.0 <= light.0 && r.start.1 <= light.1 && r.end.0 >= light.0 && r.end.1 >= light.1
        })
        .map(|r| r.action)
        .fold(false, |acc, action| {
            if action == Action::On {
                true
            } else if action == Action::Off {
                false
            } else {
                !acc
            }
        })
}

fn input_to_light_run(input: String) -> LightRun {
    let end = input.split_whitespace().last().unwrap();
    let start = input
        .split_whitespace()
        .nth(input.split_whitespace().count() - 3)
        .unwrap();

    let start = start.split_once(",").unwrap();
    let start = (start.0.parse().unwrap(), start.1.parse().unwrap());
    let end = end.split_once(",").unwrap();
    let end = (end.0.parse().unwrap(), end.1.parse().unwrap());

    LightRun {
        action: if input.starts_with("turn on") {
            Action::On
        } else if input.starts_with("turn off") {
            Action::Off
        } else {
            Action::Toggle
        },
        start,
        end,
    }
}
