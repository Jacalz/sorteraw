use anyhow::Result;
use clap::Parser;
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
    for entry in fs::read_dir(&args.src)? {
        let entry = entry?;
        let path = entry.path();

        let time: OffsetDateTime = entry.metadata()?.modified()?.into();
        let dir = args.dst.join(time.format(&date_format)?);
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let new_location = dir.join(path.file_name().expect("file name not found"));
        fs::copy(&path, &new_location)?;
    }
    Ok(())
}
