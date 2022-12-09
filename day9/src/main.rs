use clap::Parser;
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
}

#[derive(PartialEq, Eq, Clone)]
struct City {
    name: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    // First lets read in the nodes and edges
    let mut cities = HashSet::new();
    let mut edges = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Failed to parse line");
        let (line_cities, distance) = line.split_once('=').unwrap();
        let (city_a, city_b) = line_cities.split_once(" to ").unwrap();

        let distance: u32 = distance
            .trim()
            .parse()
            .expect("Failed to parse distance as a number");
        let city_a = city_a.trim().to_string();
        let city_b = city_b.trim().to_string();

        cities.insert(city_a.clone());
        cities.insert(city_b.clone());
        edges.insert((city_a.clone(), city_b.clone()), distance);
        edges.insert((city_b, city_a), distance);
    }

    let shortest_distance = find_shortest_distance(&cities, &edges);
    println!("Shortest distance: {}", shortest_distance);
}

struct SearchState {
    remaining_cities: Vec<String>,
    current_city: String,
    distance_traveled: u32,
}

fn find_shortest_distance(cities: &HashSet<String>, edges: &HashMap<(String, String), u32>) -> u32 {
    // Now we want to use A* to do minimal work until we find a solution
    let mut search_states: Vec<SearchState> = cities
        .iter()
        .map(|c| SearchState {
            current_city: c.clone(),
            remaining_cities: cities.iter().filter(|ic| **ic != *c).cloned().collect(),
            distance_traveled: 0,
        })
        .collect();

    loop {
        let (current_index, current_state) = search_states
            .iter_mut()
            .enumerate()
            .min_by(|a, b| a.1.distance_traveled.cmp(&b.1.distance_traveled))
            .unwrap();

        if current_state.remaining_cities.is_empty() {
            return current_state.distance_traveled;
        }

        let mut new_states: Vec<SearchState> = current_state
            .remaining_cities
            .iter()
            .enumerate()
            .map(|(rc_index, c)| {
                let mut remaining_cities = current_state.remaining_cities.clone();
                remaining_cities.remove(rc_index);
                SearchState {
                    current_city: c.clone(),
                    remaining_cities,
                    distance_traveled: current_state.distance_traveled
                        + edges[&(current_state.current_city.clone(), c.clone())],
                }
            })
            .collect();

        search_states.remove(current_index);
        search_states.append(&mut new_states);
    }
}
