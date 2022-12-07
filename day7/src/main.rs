use clap::Parser;
use core::panic;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    ops::{BitAnd, BitOr, Not, Shl, Shr},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    data_file: String,
}

#[derive(PartialEq, Clone, Copy)]
enum Operator {
    None,
    AND,
    LSHIFT,
    RSHIFT,
    NOT,
    OR,
}

#[derive(PartialEq, Clone)]
enum Operand {
    Immediate(u16),
    Wire(String),
}

struct Wire {
    operator: Operator,
    left_operand: Operand,
    right_operand: Operand,
    target: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut unresolved_wires: Vec<Wire> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let components: Vec<&str> = line.split(" ").collect();

        if components.len() == 3 {
            // Direct equals
            let operand_string = components.get(0).unwrap();
            let operand_value = operand_string.parse();

            let wire_name = components.get(2).unwrap().to_string();
            unresolved_wires.push(Wire {
                operator: Operator::None,
                left_operand: if let Ok(value) = operand_value {
                    Operand::Immediate(value)
                } else {
                    Operand::Wire(operand_string.to_string())
                },
                right_operand: Operand::Immediate(0),
                target: wire_name,
            });
        } else if components.len() == 4 {
            // NOT
            let wire_name = components.get(3).unwrap().to_string();
            let operand_string = components.get(1).unwrap();
            let operand_value = operand_string.parse::<u16>();
            unresolved_wires.push(Wire {
                operator: Operator::NOT,
                left_operand: if let Ok(value) = operand_value {
                    Operand::Immediate(value)
                } else {
                    Operand::Wire(operand_string.to_string())
                },
                // Right operand is unused for NOT
                right_operand: Operand::Immediate(0),
                target: wire_name,
            });
        } else {
            let left_value = components.get(0).unwrap().parse::<u16>();
            let right_value = components.get(2).unwrap().parse::<u16>();

            let wire_name = components.get(4).unwrap().to_string();
            let wire = Wire {
                operator: get_operator(components.get(1).unwrap()),
                left_operand: if left_value.is_ok() {
                    Operand::Immediate(left_value.unwrap())
                } else {
                    Operand::Wire(components.get(0).unwrap().to_string())
                },
                right_operand: if right_value.is_ok() {
                    Operand::Immediate(right_value.unwrap())
                } else {
                    Operand::Wire(components.get(2).unwrap().to_string())
                },
                target: wire_name.clone(),
            };

            unresolved_wires.push(wire);
        }
    }

    let resolved_wires = resolve_all_wires(&unresolved_wires);

    let a = *resolved_wires.get("a").unwrap();
    println!("Wire a is {}", a);

    println!("Overriding b with {}", a);
    let b_wire = unresolved_wires
        .iter_mut()
        .find(|wire| wire.target == "b")
        .unwrap();
    b_wire.left_operand = Operand::Immediate(a);
    b_wire.operator = Operator::None;
    b_wire.right_operand = Operand::Immediate(0);
    let resolved_wires = resolve_all_wires(&unresolved_wires);

    let a = *resolved_wires.get("a").unwrap();
    println!("Wire a is {}", a);
}

fn resolve_all_wires(unresolved_wires: &Vec<Wire>) -> HashMap<String, u16> {
    let mut resolved_wires: HashMap<String, u16> = HashMap::new();
    while !resolved_wires.contains_key("a") {
        for wire in unresolved_wires.iter() {
            if resolved_wires.contains_key(&wire.target) {
                continue;
            }
            let left_value = match &wire.left_operand {
                Operand::Wire(left) => {
                    if resolved_wires.contains_key(left) {
                        *resolved_wires.get(left).unwrap()
                    } else {
                        continue;
                    }
                }
                Operand::Immediate(value) => *value,
            };

            let right_value = match &wire.right_operand {
                Operand::Wire(right) => {
                    if resolved_wires.contains_key(right) {
                        *resolved_wires.get(right).unwrap()
                    } else {
                        continue;
                    }
                }
                Operand::Immediate(value) => *value,
            };

            resolved_wires.insert(
                wire.target.clone(),
                resolve_wire(wire.operator, left_value, right_value),
            );
        }
    }

    resolved_wires
}

fn resolve_wire(operator: Operator, left_value: u16, right_value: u16) -> u16 {
    let resolution = match operator {
        Operator::AND => left_value.bitand(right_value),
        Operator::OR => left_value.bitor(right_value),
        Operator::LSHIFT => left_value.shl(right_value),
        Operator::RSHIFT => left_value.shr(right_value),
        Operator::NOT => left_value.not(),
        Operator::None => left_value,
    };

    resolution
}

fn get_operator(input: &str) -> Operator {
    match input {
        "AND" => Operator::AND,
        "LSHIFT" => Operator::LSHIFT,
        "RSHIFT" => Operator::RSHIFT,
        "NOT" => Operator::NOT,
        "OR" => Operator::OR,
        other => panic!("Unexpected operator: {}", other),
    }
}
