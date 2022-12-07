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

#[derive(PartialEq)]
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

    let mut resolved_wires: HashMap<String, u16> = HashMap::new();
    let mut unresolved_wires: Vec<Wire> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let components: Vec<&str> = line.split(" ").collect();

        if components.len() == 3 {
            // Direct equals
            let operand_string = components.get(0).unwrap();
            let operand_value = operand_string.parse();

            let wire_name = components.get(2).unwrap().to_string();
            if operand_value.is_ok() {
                resolved_wires.insert(wire_name, operand_value.unwrap());
            } else {
                unresolved_wires.push(Wire {
                    operator: Operator::None,
                    left_operand: Operand::Wire(operand_string.to_string()),
                    right_operand: Operand::Immediate(0),
                    target: wire_name,
                });
            }
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

    while !resolved_wires.contains_key("a") {
        for wire in unresolved_wires.iter_mut() {
            if resolved_wires.contains_key(&wire.target) {
                continue;
            }
            match &wire.left_operand {
                Operand::Wire(left) => {
                    if resolved_wires.contains_key(left) {
                        wire.left_operand = Operand::Immediate(*resolved_wires.get(left).unwrap());
                    } else {
                        continue;
                    }
                }
                Operand::Immediate(_) => (),
            }

            match &wire.right_operand {
                Operand::Wire(right) => {
                    if resolved_wires.contains_key(right) {
                        wire.right_operand =
                            Operand::Immediate(*resolved_wires.get(right).unwrap());
                    } else {
                        continue;
                    }
                }
                Operand::Immediate(_) => (),
            }

            resolved_wires.insert(wire.target.clone(), resolve_wire(wire));
        }
    }

    println!("Wire a is {}", resolved_wires.get("a").unwrap());
}

fn resolve_wire(wire: &Wire) -> u16 {
    let left_value = if let Operand::Immediate(left_value) = wire.left_operand {
        left_value
    } else {
        panic!("Unexpected value")
    };

    let right_value = if let Operand::Immediate(right_value) = wire.right_operand {
        right_value
    } else {
        panic!("Unexpected value")
    };

    let resolution = match wire.operator {
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
