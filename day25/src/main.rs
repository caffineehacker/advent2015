use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    row: u64,
    #[arg(long)]
    column: u64,
    #[arg(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    // First find the number entry we're dealing with
    let column_start: u64 = (1..=args.column).sum();
    println!("CS: {}", column_start);
    // FIXME FIXME FIXME: The row offset is not working, column is correct
    let value_index: u64 = column_start + (args.column..(args.column + args.row - 1)).sum::<u64>();

    println!("Value index: {}", value_index);

    let starting_value = 20151125;
    let mut value: u64 = starting_value;
    for _ in 1..value_index {
        value = (value * 252533) % 33554393;

        if args.debug {
            println!("{}", value);
        }
    }

    println!("Part 1: {}", value);
}
