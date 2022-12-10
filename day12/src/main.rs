use clap::Parser;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(PartialEq, Eq)]
enum JSONValue {
    Null,
    String(String),
    Number(i32),
    Object(HashMap<String, JSONValue>),
    Array(Vec<JSONValue>),
    Bool(bool),
}

impl Display for JSONValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            JSONValue::Null => todo!(),
            JSONValue::String(s) => formatter.write_fmt(format_args!("\"{}\"", s)),
            JSONValue::Number(n) => formatter.write_fmt(format_args!("{}", n)),
            JSONValue::Object(o) => {
                formatter.write_str("{")?;
                for entry in o.iter() {
                    formatter.write_fmt(format_args!("\"{}\": {},", entry.0, entry.1))?;
                }
                formatter.write_str("}")?;
                Ok(())
            }
            JSONValue::Array(arr) => {
                formatter.write_str("[")?;
                for entry in arr.iter() {
                    formatter.write_fmt(format_args!("{},", entry))?;
                }
                formatter.write_str("]")?;
                Ok(())
            }
            JSONValue::Bool(_) => todo!(),
        }
    }
}

fn main() {
    let args = Args::parse();

    let data = fs::read_to_string(&args.data_file).expect("Failed to open file");

    // I could use an existing JSON parser, but where's the fun in that?
    let json_object = parse_json(&data.chars().collect(), 0).1;

    println!("Part 1: {}", sum(&json_object));
    println!("Part 2: {}", sum_part2(&json_object));
}

fn sum(value: &JSONValue) -> i32 {
    match value {
        JSONValue::Null => todo!(),
        JSONValue::String(_) => 0,
        JSONValue::Number(n) => *n,
        JSONValue::Object(o) => o.iter().map(|e| sum(e.1)).sum(),
        JSONValue::Array(arr) => arr.iter().map(|e| sum(e)).sum(),
        JSONValue::Bool(_) => todo!(),
    }
}

fn sum_part2(value: &JSONValue) -> i32 {
    match value {
        JSONValue::Null => todo!(),
        JSONValue::String(_) => 0,
        JSONValue::Number(n) => *n,
        JSONValue::Object(o) => {
            if o.iter().any(|e| {
                if let JSONValue::String(s) = e.1 {
                    s == "red"
                } else {
                    false
                }
            }) {
                return 0;
            }
            o.iter().map(|e| sum_part2(e.1)).sum()
        }
        JSONValue::Array(arr) => arr.iter().map(|e| sum_part2(e)).sum(),
        JSONValue::Bool(_) => todo!(),
    }
}

fn parse_json(json: &Vec<char>, index: usize) -> (usize, JSONValue) {
    match json[index] {
        '[' => parse_json_array(&json, index),
        '{' => parse_json_object(&json, index),
        '"' => parse_json_string(&json, index),
        '0'..='9' | '-' => parse_json_number(&json, index),
        _ => panic!("Unexpected char"),
    }
}

fn parse_json_array(json: &Vec<char>, index: usize) -> (usize, JSONValue) {
    let mut index = index;
    // Eat the opening bracket
    index += 1;

    let mut array: Vec<JSONValue> = Vec::new();
    while json[index] != ']' {
        let (new_index, value) = parse_json(json, index);
        array.push(value);
        index = new_index;

        if json[index] == ',' {
            index += 1;
        }
    }
    index += 1; // Eat the closing bracket

    (index, JSONValue::Array(array))
}

fn parse_json_object(json: &Vec<char>, index: usize) -> (usize, JSONValue) {
    let mut index = index;
    // Eat the opening brace
    index += 1;

    let mut object = HashMap::new();
    while json[index] != '}' {
        let (new_index, key) = parse_json(json, index);
        let key = match key {
            JSONValue::String(k) => k,
            _ => panic!("A non-string object key was found"),
        };
        if json[new_index] != ':' {
            panic!("Unexpected end of object entry");
        }
        index = new_index + 1; // Eat the colon
        let (new_index, value) = parse_json(json, index);
        object.insert(key, value);
        index = new_index;

        if json[index] == ',' {
            index += 1;
        }
    }
    index += 1; // Eat the closing brace

    (index, JSONValue::Object(object))
}

fn parse_json_string(json: &Vec<char>, index: usize) -> (usize, JSONValue) {
    let mut index = index;
    index += 1; // Eat the open quote
    let mut result = "".to_string();

    while json[index] != '"' {
        // FIXME FIXME FIXME
        // Handle escape sequences...
        result.push(json[index]);
        index += 1;
    }
    index += 1; // Eat the closing quote

    (index, JSONValue::String(result))
}

fn parse_json_number(json: &Vec<char>, index: usize) -> (usize, JSONValue) {
    let mut index = index;

    let mut number: i32 = 0;
    let is_negative = if json[index] == '-' {
        index += 1;
        true
    } else {
        false
    };

    while ('0'..='9').contains(&json[index]) {
        number *= 10;
        number += (json[index] as i32) - ('0' as i32);
        index += 1;
    }

    if is_negative {
        number *= -1;
    }

    (index, JSONValue::Number(number))
}
