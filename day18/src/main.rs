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
    let lights: HashMap<(usize, usize), bool> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| ((x, y), c == '#'))
                .collect::<Vec<((usize, usize), bool)>>()
        })
        .collect();

    solve_part1(lights.clone());
}

fn solve_part1(mut lights: HashMap<(usize, usize), bool>) {
    for _ in 0..100 {
        let mut new_lights = lights.clone();
        new_lights.iter_mut().for_each(|(pos, l)| {
            let mut neighbors_lit = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x != 0 || y != 0 && pos.0 as isize + x >= 0 && pos.1 as isize + y >= 0 {
                        neighbors_lit += lights
                            .get(&((pos.0 as isize + x) as usize, (pos.1 as isize + y) as usize))
                            .map(|l| if *l { 1 } else { 0 })
                            .unwrap_or(0);
                    }
                }
            }

            if *l && !(neighbors_lit == 2 || neighbors_lit == 3) {
                *l = false;
            } else if !*l && neighbors_lit == 3 {
                *l = true;
            }
        });
        lights = new_lights;
    }

    let number_lit = lights.values().filter(|l| **l).count();
    println!("Part 1: {}", number_lit);
}
