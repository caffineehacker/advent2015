use clap::Parser;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
    #[arg(long)]
    include_me: bool,
}

#[derive(Clone)]
struct State {
    ordering: Vec<String>,
    remaining: HashSet<String>,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut preferences: HashMap<(String, String), i32> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_preferences)
        .collect();

    let mut names: HashSet<String> = preferences.keys().map(|k| k.0.to_string()).collect();
    if args.include_me {
        names.iter().for_each(|n| {
            preferences.insert(("MEEEE".to_string(), n.to_string()), 0);
            preferences.insert((n.to_string(), "MEEEE".to_string()), 0);
        });
        names.insert("MEEEE".to_string());
    }
    let mut states = vec![State {
        ordering: Vec::new(),
        remaining: names.clone(),
    }];

    while states[0].remaining.len() > 0 {
        states = states
            .par_iter()
            .flat_map(|s| {
                s.remaining
                    .iter()
                    .map(|person| {
                        let mut ordering = s.ordering.clone();
                        ordering.push(person.clone());
                        let mut remaining = s.remaining.clone();
                        remaining.remove(person);
                        State {
                            ordering,
                            remaining,
                        }
                    })
                    .collect::<Vec<State>>()
            })
            .collect();
    }

    // Now states contains only final states
    let best_state_score: i32 = states
        .par_iter()
        .map(|state| {
            score_state(&state.ordering, &preferences)
                + score_state(
                    &state.ordering.iter().rev().cloned().collect(),
                    &preferences,
                )
        })
        .max()
        .unwrap();

    println!("Best score: {}", best_state_score);
}

fn score_state(ordering: &Vec<String>, preferences: &HashMap<(String, String), i32>) -> i32 {
    let mut score = 0;

    for i in 0..ordering.len() {
        let person = ordering[i].clone();
        let target = ordering[if i == (ordering.len() - 1) { 0 } else { i + 1 }].clone();

        score += preferences.get(&(person, target)).unwrap();
    }

    score
}

fn parse_preferences(input: String) -> ((String, String), i32) {
    let input: Vec<&str> = input.split_whitespace().collect();
    let first_name = input[0].to_string();
    let second_name = input[10].trim_end_matches('.').to_string();
    let amount = input[3].parse::<i32>().unwrap() * if input[2] == "lose" { -1 } else { 1 };

    ((first_name, second_name), amount)
}
