use std::error::Error;

use clap::Parser;

use fsz::{total_size, print_fmt};

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Accumulate file size by pattern.")]
struct Args {
    root: String,

    #[arg(short, long)]
    pattern: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let size = total_size(&args.root, &args.pattern)?;
    print_fmt(size)?;

    Ok(())
}
