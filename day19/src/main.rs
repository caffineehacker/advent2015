use clap::Parser;
use multimap::MultiMap;
use rayon::prelude::*;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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

    // Approach is going to be to tokenize the input.
    // Each letter we see will be added to a hash map with a unique number for each one
    // Then the transforms can be from # -> Vec<#> and the input string can be Vec<#>

    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let mut next_key = 0;
    let mut mappings: HashMap<String, u32> = HashMap::new();
    let converters: MultiMap<u32, Vec<u32>> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_conversion(line, &mut mappings, &mut next_key))
        .collect();

    let input = tokenize(
        lines
            .iter()
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .next()
            .unwrap(),
        &mut mappings,
        &mut next_key,
    );

    do_part1(&converters, &input);
    do_part2(&converters, &input, &vec![mappings["e"]]);
}

fn do_part1(converters: &MultiMap<u32, Vec<u32>>, input: &Vec<u32>) {
    let elements = get_elements_from_one_conversion(converters, input);

    println!("Unique elements from one conversion: {}", elements.len());
}

fn get_elements_from_one_conversion(
    converters: &MultiMap<u32, Vec<u32>>,
    input: &Vec<u32>,
) -> HashSet<Vec<u32>> {
    let mut elements = HashSet::new();

    for i in 0..input.len() {
        if !converters.contains_key(&input[i]) {
            continue;
        }
        for c in converters.get_vec(&input[i]).unwrap().iter() {
            let mut result = input[0..i].to_vec();
            result.append(&mut c.clone());
            result.append(&mut input[(i + 1)..].to_vec());
            elements.insert(result);
        }
    }

    elements
}

fn get_elements_from_one_conversion_part2(
    converters: &HashMap<Vec<u32>, u32>,
    input: &Vec<u32>,
) -> HashSet<Vec<u32>> {
    let longest_convert = converters.keys().map(|k| k.len()).max().unwrap();
    let mut elements = HashSet::new();
    for i in 0..input.len() {
        for j in i..=(i + longest_convert).min(input.len() - 1) {
            if i + j >= input.len() {
                break;
            }

            if let Some(c) = converters.get(&input[i..=j].to_vec()) {
                let mut result = input[0..i].to_vec();
                result.push(*c);
                result.append(&mut input[(j + 1)..].to_vec());
                elements.insert(result);
            }
        }
    }

    elements
}

fn do_part2(converters: &MultiMap<u32, Vec<u32>>, target: &Vec<u32>, starting_element: &Vec<u32>) {
    let mut seen_states = HashSet::new();
    seen_states.insert(target.clone());

    let mut current_states = BinaryHeap::new();
    current_states.push((1000 - target.len(), target.clone(), 0));

    let converters: HashMap<Vec<u32>, u32> = converters
        .iter_all()
        .flat_map(|(k, vs)| vs.iter().map(|v| (v.clone(), *k)))
        .collect();

    while !current_states.is_empty() {
        let current_state = current_states.pop().unwrap();

        let mut new_states: BinaryHeap<(usize, Vec<u32>, u32)> =
            get_elements_from_one_conversion_part2(&converters, &current_state.1)
                .into_iter()
                .map(|s| (1000 - s.len(), s, current_state.2 + 1))
                .filter(|m| seen_states.insert(m.1.clone()))
                .collect();

        if let Some(s) = new_states.iter().find(|s| *s.1 == *starting_element) {
            println!("Part 2: {}", s.2);
            return;
        }

        current_states.append(&mut new_states);
    }
}

fn parse_conversion(
    line: &str,
    mappings: &mut HashMap<String, u32>,
    next_key: &mut u32,
) -> (u32, Vec<u32>) {
    let (left, right) = line.split_once(" => ").unwrap();

    let left = if mappings.contains_key(left) {
        mappings[left]
    } else {
        let left_key = *next_key;
        mappings.insert(left.to_string(), *next_key);
        *next_key += 1;
        left_key
    };

    (left, tokenize(right, mappings, next_key))
}

fn tokenize(input: &str, mappings: &mut HashMap<String, u32>, next_key: &mut u32) -> Vec<u32> {
    let mut values = Vec::new();
    let mut index = 0;
    while index < input.len() {
        let mut element = &input[index..=index];
        if index + 1 < input.len() && input.chars().nth(index + 1).unwrap() >= 'a' {
            // This is a two char element
            element = &input[index..=(index + 1)];
            index += 1;
        }

        let value = if mappings.contains_key(element) {
            mappings[element]
        } else {
            let key = *next_key;
            mappings.insert(element.to_string(), *next_key);
            *next_key += 1;
            key
        };
        values.push(value);

        index += 1;
    }

    values
}
