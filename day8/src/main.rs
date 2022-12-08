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

    let mut code_length = 0;
    let mut representation_length = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        code_length += line.chars().count();

        let parsed_line = parse_line(&line);
        println!("\"{}\" -> \"{}\"", line, parsed_line);
        representation_length += parsed_line.chars().count();
    }

    println!("Code length: {}", code_length);
    println!("Storage size: {}", representation_length);
    println!("Diff: {}", code_length - representation_length);
}

fn parse_line(line: &str) -> String {
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
