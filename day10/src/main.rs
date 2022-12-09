use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let mut state = string_to_rle(args.input);
    for step in 0..40 {
        state = rle(state);
        println!(
            "Step {}, length {}",
            step,
            state.iter().map(|s| s.0).sum::<u32>()
        );
    }

    println!(
        "After 40 cycles length is {}",
        state.iter().map(|s| s.0).sum::<u32>()
    );

    for step in 0..10 {
        state = rle(state);
        println!(
            "Step {}, length {}",
            step + 40,
            state.iter().map(|s| s.0).sum::<u32>()
        );
    }

    println!(
        "After 50 cycles length is {}",
        state.iter().map(|s| s.0).sum::<u32>()
    );
}

fn string_to_rle(input: String) -> Vec<(u32, u32)> {
    let mut index = 0;
    let mut output = Vec::new();

    while index < input.len() {
        let number = input.chars().nth(index).unwrap();
        let count = input
            .chars()
            .skip(index)
            .take_while(|c| *c == number)
            .count();

        output.push((count as u32, number.to_string().parse::<u32>().unwrap()));
        index += count;
    }

    output
}

fn rle(state: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut index = 0;
    let mut output = Vec::new();

    let state: Vec<(u32, u32)> = state
        .iter()
        .flat_map(|s| vec![(1u32, s.0), (1u32, s.1)])
        .collect();

    while index < state.len() {
        let current = state[index];
        // If current is (3, 1) -> "111" then the next output would be (1, 3), (1, 1) -> "31"
        let new_count: u32 = state
            .iter()
            .skip(index)
            .take_while(|c| c.1 == current.1)
            .map(|s| s.0)
            .sum();

        output.push((new_count as u32, current.1));
        index += state
            .iter()
            .skip(index)
            .take_while(|c| c.1 == current.1)
            .count();
    }

    output
}
