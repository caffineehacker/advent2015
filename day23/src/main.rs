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
    #[arg(long)]
    debug: bool,
    #[arg(long)]
    start_a: u32,
}

enum Register {
    A,
    B,
}

impl Into<Register> for &str {
    fn into(self) -> Register {
        match self {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("Unexpected register"),
        }
    }
}

enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i32),
    JumpIfEven(Register, i32),
    JumpIfOne(Register, i32),
}

impl Into<Instruction> for String {
    fn into(self) -> Instruction {
        let components: Vec<&str> = self.split_whitespace().collect();

        match components[0] {
            "hlf" => Instruction::Half(components[1].into()),
            "tpl" => Instruction::Triple(components[1].into()),
            "inc" => Instruction::Increment(components[1].into()),
            "jmp" => Instruction::Jump(components[1].parse().unwrap()),
            "jie" => Instruction::JumpIfEven(
                components[1].trim_end_matches(",").into(),
                components[2].parse().unwrap(),
            ),
            "jio" => Instruction::JumpIfOne(
                components[1].trim_end_matches(",").into(),
                components[2].parse().unwrap(),
            ),
            _ => panic!("Unexpected instruction"),
        }
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.expect("Failed to read line").into())
        .collect();

    let mut pc = 0;
    let mut register_a: u32 = args.start_a;
    let mut register_b: u32 = 0;
    while pc < instructions.len() {
        match &instructions[pc] {
            Instruction::Half(register) => match register {
                Register::A => register_a /= 2,
                Register::B => register_b /= 2,
            },
            Instruction::Triple(register) => match register {
                Register::A => register_a *= 3,
                Register::B => register_b *= 3,
            },
            Instruction::Increment(register) => match register {
                Register::A => register_a += 1,
                Register::B => register_b += 1,
            },
            Instruction::Jump(offset) => pc = (pc as i32 + offset - 1) as usize,
            Instruction::JumpIfEven(register, offset) => match register {
                Register::A => {
                    if register_a % 2 == 0 {
                        pc = (pc as i32 + offset - 1) as usize
                    }
                }
                Register::B => {
                    if register_b % 2 == 0 {
                        pc = (pc as i32 + offset - 1) as usize
                    }
                }
            },
            Instruction::JumpIfOne(register, offset) => match register {
                Register::A => {
                    if register_a == 1 {
                        pc = (pc as i32 + offset - 1) as usize
                    }
                }
                Register::B => {
                    if register_b == 1 {
                        pc = (pc as i32 + offset - 1) as usize
                    }
                }
            },
        }
        pc += 1;

        if args.debug {
            println!("PC: {}, A: {}, B: {}", pc, register_a, register_b);
        }
    }

    println!("A: {}, B: {}", register_a, register_b);
}
