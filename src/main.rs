use anyhow::Result;
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use time::{OffsetDateTime, format_description};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    src: PathBuf,
    dst: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let date_format = format_description::parse("[year]-[month]-[day]")?;
    let mut seen_dirs = HashSet::new();

    for entry in fs::read_dir(&args.src)? {
        let entry = entry?;
        let path = entry.path();

        let time: OffsetDateTime = entry.metadata()?.modified()?.into();
        let dir = args.dst.join(time.format(&date_format)?);
        let new_location = dir.join(path.file_name().expect("file name not found"));
        if !seen_dirs.contains(&dir) {
            fs::create_dir(&dir)?;
            seen_dirs.insert(dir);
        }

        fs::copy(&path, &new_location)?;
    }
    Ok(())
}
