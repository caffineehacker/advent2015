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
        .map(|l| l.expect("Failed to read line"))
        .collect();

    let mut encoded_length = 0;
    let mut storage_length = 0;

    for line in lines.iter() {
        encoded_length += line.chars().count();

        let parsed_line = parse_line_part1(&line);
        println!("\"{}\" -> \"{}\"", line, parsed_line);
        storage_length += parsed_line.chars().count();
    }

    println!("Part 1");
    println!("Enoded length: {}", encoded_length);
    println!("Storage size: {}", storage_length);
    println!("Diff: {}", encoded_length - storage_length);

    println!("-----------------------");

    storage_length = 0;
    encoded_length = 0;
    for line in lines.iter() {
        storage_length += line.chars().count();

        let encoded_line = encode_line_part2(&line);
        println!("\"{}\" -> \"{}\"", line, encoded_line);
        encoded_length += encoded_line.chars().count();
    }

    println!("Part 2");
    println!("Encoded length: {}", encoded_length);
    println!("Storage size: {}", storage_length);
    println!("Diff: {}", encoded_length - storage_length);
}

fn encode_line_part2(line: &str) -> String {
    let line = line.to_string();
    let line = line.replace("\\", "\\\\");
    let line = line.replace("\"", "\\\"");

    "\"".to_string() + &line + "\""
}

fn parse_line_part1(line: &str) -> String {
    let mut line = line.get(1..(line.len() - 1)).unwrap().to_string();

    let mut starting_point = 0;
    while line.chars().skip(starting_point).any(|c| c == '\\') {
        let index = line
            .get(starting_point..line.len())
            .unwrap()
            .find('\\')
            .unwrap()
            + starting_point;
        starting_point = index + 1;
        match line.chars().nth(index + 1).expect("No char after \"\\\"") {
            '\\' => {
                line.remove(index);
            }
            '\"' => {
                // Remove the "\" to just leave the quote
                line.remove(index);
            }
            'x' => {
                // TODO: If needed, decode the char
                line.remove(index);
                line.remove(index);
                line.remove(index);
            }
            _ => panic!("Unexpected escape sequence"),
        };
    }

    line
}
