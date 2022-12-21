use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: usize,
}

fn main() {
    let args = Args::parse();

    // Part 1
    let mut houses = vec![0; 1000000];
    for i in 0..houses.len() {
        // Unroll the first part of the loop so we can do this check
        houses[i] += (i + 1) * 10;
        if houses[i] >= args.input {
            println!("Part 1: {}", i + 1);
            break;
        }

        let mut current = i + (i + 1);
        while current < houses.len() {
            houses[current] += (i + 1) * 10;
            current += i + 1;
        }
    }
}
