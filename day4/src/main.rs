use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let mut current_guess = 0;
    while !format!(
        "{:x}",
        md5::compute(args.input.clone() + &current_guess.to_string())
    )
    .starts_with("00000")
    {
        current_guess += 1;
    }

    println!("Part 1 result: {}", current_guess);

    // We don't reset current_guess since we're now looking for more zeroes and we would not have skipped a solution to this part above
    while !format!(
        "{:x}",
        md5::compute(args.input.clone() + &current_guess.to_string())
    )
    .starts_with("000000")
    {
        current_guess += 1;
    }

    println!("Part 2 result: {}", current_guess);
}
