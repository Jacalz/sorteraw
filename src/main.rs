use anyhow::{Result, anyhow};
use clap::Parser;
use rayon::{ThreadPoolBuilder, prelude::*};
use std::collections::HashSet;
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

    Sorter::new(args).run()
}

struct Sorter {
    args: Args,
    seen_dirs: HashSet<PathBuf>,
}

impl Sorter {
    fn new(args: Args) -> Self {
        Sorter {
            args,
            seen_dirs: HashSet::new(),
        }
    }
}

impl Sorter {
    fn run(&mut self) -> Result<()> {
        let files = fs::read_dir(&self.args.src)?
            .map(|entry| -> Result<Option<(PathBuf, PathBuf)>> {
                self.init_directory_and_paths(&entry?)
            })
            .collect::<Result<Vec<_>>>()?;

        ThreadPoolBuilder::new()
            .build()?
            .install(|| -> Result<()> { self.sort(&files) })
    }

    fn init_directory_and_paths(&mut self, entry: &DirEntry) -> Result<Option<(PathBuf, PathBuf)>> {
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

        if !self.seen_dirs.contains(&dir) {
            fs::create_dir(&dir)?;
            self.seen_dirs.insert(dir);
        }

        Ok(Some((old_path, new_path)))
    }

    fn sort(&self, files: &Vec<Option<(PathBuf, PathBuf)>>) -> Result<()> {
        files.par_iter().try_for_each(|pair| -> Result<()> {
            if let Some((old_path, new_path)) = pair {
                self.move_into_place(old_path, new_path)?;
            }
            Ok(())
        })
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
