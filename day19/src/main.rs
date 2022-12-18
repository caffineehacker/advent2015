use clap::Parser;
use multimap::MultiMap;
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
}

fn do_part1(converters: &MultiMap<u32, Vec<u32>>, input: &Vec<u32>) {
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

    println!("Unique elements from one conversion: {}", elements.len());
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
