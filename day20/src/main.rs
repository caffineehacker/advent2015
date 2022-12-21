use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: usize,
}

fn main() {
    let args = Args::parse();

    do_part1(args.input);
    do_part2(args.input);
}

fn do_part1(input: usize) {
    let mut houses = vec![0; 1000000];
    for i in 0..houses.len() {
        // Unroll the first part of the loop so we can do this check
        houses[i] += (i + 1) * 10;
        if houses[i] >= input {
            println!("Part 1: {}", i + 1);
            return;
        }

        let mut current = i + (i + 1);
        while current < houses.len() {
            houses[current] += (i + 1) * 10;
            current += i + 1;
        }
    }
}

fn do_part2(input: usize) {
    let mut houses = vec![0; 1000000];
    for i in 0..houses.len() {
        // Unroll the first part of the loop so we can do this check
        houses[i] += (i + 1) * 11;
        if houses[i] >= input {
            println!("Part 2: {}", i + 1);
            return;
        }

        let mut current = i + (i + 1);
        for _ in 1..50 {
            if current >= houses.len() {
                break;
            }
            houses[current] += (i + 1) * 11;
            current += i + 1;
        }
    }
}
