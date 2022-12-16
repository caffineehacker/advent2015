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

#[derive(Default)]
struct Sue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Sue {
    fn part1_partially_equal(&self, other: &Self) -> bool {
        if self.children.is_some()
            && other.children.is_some()
            && self.children.unwrap() != other.children.unwrap()
        {
            return false;
        }

        if self.cats.is_some() && other.cats.is_some() && self.cats.unwrap() != other.cats.unwrap()
        {
            return false;
        }

        if self.samoyeds.is_some()
            && other.samoyeds.is_some()
            && self.samoyeds.unwrap() != other.samoyeds.unwrap()
        {
            return false;
        }

        if self.pomeranians.is_some()
            && other.pomeranians.is_some()
            && self.pomeranians.unwrap() != other.pomeranians.unwrap()
        {
            return false;
        }

        if self.akitas.is_some()
            && other.akitas.is_some()
            && self.akitas.unwrap() != other.akitas.unwrap()
        {
            return false;
        }

        if self.vizslas.is_some()
            && other.vizslas.is_some()
            && self.vizslas.unwrap() != other.vizslas.unwrap()
        {
            return false;
        }

        if self.goldfish.is_some()
            && other.goldfish.is_some()
            && self.goldfish.unwrap() != other.goldfish.unwrap()
        {
            return false;
        }

        if self.trees.is_some()
            && other.trees.is_some()
            && self.trees.unwrap() != other.trees.unwrap()
        {
            return false;
        }

        if self.cars.is_some() && other.cars.is_some() && self.cars.unwrap() != other.cars.unwrap()
        {
            return false;
        }

        if self.perfumes.is_some()
            && other.perfumes.is_some()
            && self.perfumes.unwrap() != other.perfumes.unwrap()
        {
            return false;
        }

        true
    }

    // The target should be the self
    fn part2_partially_equal(&self, other: &Self) -> bool {
        if self.children.is_some()
            && other.children.is_some()
            && self.children.unwrap() != other.children.unwrap()
        {
            return false;
        }

        if self.cats.is_some() && other.cats.is_some() && self.cats.unwrap() >= other.cats.unwrap()
        {
            return false;
        }

        if self.samoyeds.is_some()
            && other.samoyeds.is_some()
            && self.samoyeds.unwrap() != other.samoyeds.unwrap()
        {
            return false;
        }

        if self.pomeranians.is_some()
            && other.pomeranians.is_some()
            && self.pomeranians.unwrap() <= other.pomeranians.unwrap()
        {
            return false;
        }

        if self.akitas.is_some()
            && other.akitas.is_some()
            && self.akitas.unwrap() != other.akitas.unwrap()
        {
            return false;
        }

        if self.vizslas.is_some()
            && other.vizslas.is_some()
            && self.vizslas.unwrap() != other.vizslas.unwrap()
        {
            return false;
        }

        if self.goldfish.is_some()
            && other.goldfish.is_some()
            && self.goldfish.unwrap() <= other.goldfish.unwrap()
        {
            return false;
        }

        if self.trees.is_some()
            && other.trees.is_some()
            && self.trees.unwrap() >= other.trees.unwrap()
        {
            return false;
        }

        if self.cars.is_some() && other.cars.is_some() && self.cars.unwrap() != other.cars.unwrap()
        {
            return false;
        }

        if self.perfumes.is_some()
            && other.perfumes.is_some()
            && self.perfumes.unwrap() != other.perfumes.unwrap()
        {
            return false;
        }

        true
    }
}

fn main() {
    let args = Args::parse();

    let file = File::open(&args.data_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let target_sue = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let sues: Vec<Sue> = reader
        .lines()
        .map(|line| line.expect("Failed to parse line"))
        .map(parse_sue)
        .collect();

    let part1_matching_sue_number = sues
        .iter()
        .enumerate()
        .filter(|s| target_sue.part1_partially_equal(s.1))
        .last()
        .unwrap()
        .0
        + 1;

    println!("Part 1 matching sue number: {}", part1_matching_sue_number);

    let part2_matching_sue_number = sues
        .iter()
        .enumerate()
        .filter(|s| target_sue.part2_partially_equal(s.1))
        .last()
        .unwrap()
        .0
        + 1;

    println!("Part 2 matching sue number: {}", part2_matching_sue_number);
}

fn parse_sue(line: String) -> Sue {
    let (_, items) = line.split_once(": ").unwrap();

    let mut sue = Sue::default();

    for item in items.split(", ") {
        let (item, amount) = item.split_once(": ").unwrap();
        let amount: u32 = amount.trim_end_matches(",").parse().unwrap();
        match item {
            "children" => sue.children = Some(amount),
            "cats" => sue.cats = Some(amount),
            "samoyeds" => sue.samoyeds = Some(amount),
            "pomeranians" => sue.pomeranians = Some(amount),
            "akitas" => sue.akitas = Some(amount),
            "vizslas" => sue.vizslas = Some(amount),
            "goldfish" => sue.goldfish = Some(amount),
            "trees" => sue.trees = Some(amount),
            "cars" => sue.cars = Some(amount),
            "perfumes" => sue.perfumes = Some(amount),
            _ => panic!("Unexpected item name"),
        }
    }

    sue
}
