//! cargo-mtime-travel saves and restores the mtime attribute for files.

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
            file,
            target_dir,
            ignore,
            verbose,
        } => {
            save_mtimes(file, target_dir, ignore, verbose)?;
        }
        Commands::Restore {
            file,
            target_dir,
            verbose,
            ignore_hash,
        } => {
            restore_mtimes(file, target_dir, verbose, ignore_hash)?;
        }
    }

    Ok(())
}
