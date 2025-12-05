use std::error::Error;

use clap::Parser;

use fsz::total_size;

#[derive(Parser, Debug)]
#[command(version, about, long_about = "Accumulate file size by pattern.")]
struct Args {
    root: String,

    #[arg(short, long)]
    pattern: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    total_size(&args.root, &args.pattern)?;

    Ok(())
}
