use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    src: String,
    dst: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    for entry in fs::read_dir(&args.src)? {
        let path = entry?.path();
        dbg!(path);
    }
    Ok(())
}
