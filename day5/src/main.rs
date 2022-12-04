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

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let nice_lines_count = lines.iter().filter(is_nice).count();
    println!("Nice lines count: {}", nice_lines_count);
}

fn is_nice(input: &&String) -> bool {
    // A nice string:
    // - Contains at least three vowels (aeiou only)
    // - Contains at least one letter that appears twice in a row
    // - Does NOT contain the strings ab, cd, pq, or xy

    if input
        .chars()
        .filter(|c| vec!['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
        < 3
    {
        return false;
    }

    let (_, has_repeat) = input.chars().fold((None, None), |acc, c| match acc {
        (_, Some(_)) => acc,
        (None, None) => (Some(c), None),
        (Some(last_c), None) => (Some(c), if last_c == c { Some(c) } else { None }),
    });
    if has_repeat.is_none() {
        return false;
    }

    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
    {
        return false;
    }

    true
}
