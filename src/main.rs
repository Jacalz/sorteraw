use anyhow::{Result, anyhow};
use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use time::{OffsetDateTime, format_description};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Source directory to look for files to sort.
    src: PathBuf,
    /// Destination directory where folders will be created.
    dst: PathBuf,

    /// Move files instead of copying them.
    #[arg(short, long)]
    move_files: bool,
}

fn validate_args(args: &Args) -> Result<()> {
    if !args.src.exists() {
        return Err(anyhow!("Source directory does not exist"));
    }

    if !args.dst.exists() {
        fs::create_dir_all(&args.dst)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    validate_args(&args)?;

    let date_format = format_description::parse("[year]-[month]-[day]")?;
    let mut seen_dirs = HashSet::new();

    for entry in fs::read_dir(&args.src)? {
        let entry = entry?;

        let time: OffsetDateTime = entry.metadata()?.modified()?.into();
        let dir = args.dst.join(time.format(&date_format)?);

        let old_path = entry.path();
        let new_path = dir.join(old_path.file_name().expect("file name not found"));

        if !seen_dirs.contains(&dir) {
            fs::create_dir(&dir)?;
            seen_dirs.insert(dir);
        }

        if args.move_files {
            fs::rename(&old_path, &new_path)?;
        } else {
            fs::copy(&old_path, &new_path)?;
        }
    }
    Ok(())
}
