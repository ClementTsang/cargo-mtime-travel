//! cargo-mtime-travel stores and restores the mtime attribute for files.

mod args;
mod file_entry;
mod hash;
mod restore;
mod save;

use anyhow::Result;
use args::{Args, Commands};
use clap::Parser;
use restore::restore_mtimes;
use save::save_mtimes;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Save {
            mtime_file,
            target_dir,
            ignore,
            verbose,
        } => {
            save_mtimes(mtime_file, target_dir, ignore, verbose)?;
        }
        Commands::Restore {
            mtime_file,
            target_dir,
            verbose,
        } => {
            restore_mtimes(mtime_file, target_dir, verbose)?;
        }
    }

    Ok(())
}
