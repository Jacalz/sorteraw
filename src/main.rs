use anyhow::{Result, anyhow};
use clap::Parser;
use dashmap::DashSet;
use rayon::prelude::*;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use time::{OffsetDateTime, macros::format_description};

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] =
    format_description!("[year]-[month]-[day]");

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

fn main() -> Result<()> {
    let args = Args::parse();
    if !args.src.exists() {
        return Err(anyhow!("Source directory does not exist"));
    }
    if !args.dst.exists() {
        fs::create_dir_all(&args.dst)?;
    }

    let sorter = Sorter::new(args);
    let files = sorter.create_and_collect()?;
    sorter.sort(&files)
}

struct Sorter {
    args: Args,
    seen_dirs: DashSet<PathBuf>,
}

impl Sorter {
    fn new(args: Args) -> Self {
        Sorter {
            args,
            seen_dirs: DashSet::new(),
        }
    }
}

impl Sorter {
    fn create_and_collect(&self) -> Result<Vec<(PathBuf, PathBuf)>> {
        let files = fs::read_dir(&self.args.src)?
            .par_bridge()
            .try_fold(
                Vec::new,
                |mut acc, entry| -> Result<Vec<(PathBuf, PathBuf)>> {
                    if let Some((old, new)) = self.create_directory_if_needed(&entry?)? {
                        acc.push((old, new));
                    }
                    Ok(acc)
                },
            )
            .try_reduce(Vec::new, |mut acc1, mut acc2| {
                acc1.append(&mut acc2);
                Ok(acc1)
            })?;

        Ok(files)
    }

    fn create_directory_if_needed(&self, entry: &DirEntry) -> Result<Option<(PathBuf, PathBuf)>> {
        let time = OffsetDateTime::from(entry.metadata()?.modified()?);
        let dir = self.args.dst.join(time.format(DATE_FORMAT)?);

        let old_path = entry.path();
        if old_path.is_dir() {
            return Ok(None);
        }

        let new_path = dir.join(old_path.file_name().unwrap());
        if new_path.exists() {
            return Err(anyhow!("File {:?} already exists at destination", new_path));
        }

        if self.seen_dirs.insert(dir.clone()) {
            fs::create_dir(dir)?;
        }

        Ok(Some((old_path, new_path)))
    }

    fn sort(&self, files: &Vec<(PathBuf, PathBuf)>) -> Result<()> {
        files
            .par_iter()
            .try_for_each(|(old_path, new_path)| self.move_into_place(old_path, new_path))
    }

    fn move_into_place(&self, old_path: &PathBuf, new_path: &PathBuf) -> Result<()> {
        if self.args.move_files {
            fs::rename(old_path, new_path)?;
        } else {
            fs::copy(old_path, new_path)?;
        }

        Ok(())
    }
}
