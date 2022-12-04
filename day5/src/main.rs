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
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let nice_lines_part1_count = lines.iter().filter(is_nice_part1).count();
    println!("Part 1 nice lines count: {}", nice_lines_part1_count);

    let nice_lines_part2_count = lines.iter().filter(is_nice_part2).count();
    println!("Part 2 nice lines count: {}", nice_lines_part2_count);
}

fn is_nice_part1(input: &&String) -> bool {
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

fn is_nice_part2(input: &&String) -> bool {
    // A nice string:
    // - Contains a pair of any two letters that appears at least twice in the string without overlapping
    //      E.g. xyxy (xy) or aabcdefgaa (aa) but not aaa (aa overlaps)
    // - Contains at least one letter which repeats with exactly one letter between them
    //      E.g. xyx, abcdefeghi (efe), or aaa

    let mut found_pairs = HashMap::new();
    let input: Vec<char> = input.chars().collect();
    let mut last_char = *input.get(1).unwrap();
    let mut two_chars_ago = *input.get(0).unwrap();
    found_pairs.insert((two_chars_ago, last_char), 0);

    let mut found_repeat_letter = false;
    let mut found_repeat_pair = false;
    for (index, c) in input.iter().enumerate().skip(2) {
        if *c == two_chars_ago {
            found_repeat_letter = true;
        }

        if let Some(last_index) = found_pairs.get(&(last_char, *c)) {
            // Check if the range overlapped. The pair we're looking at started 1 index ago
            if *last_index < index - 2 {
                found_repeat_pair = true;
            }
        } else {
            found_pairs.insert((last_char, *c), index - 1);
        }

        two_chars_ago = last_char;
        last_char = *c;
    }

    found_repeat_letter && found_repeat_pair
}
