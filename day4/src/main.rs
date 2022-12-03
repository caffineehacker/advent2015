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

    println!("Result: {}", current_guess);
}
