use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// A small tool to save and restore the mtime attribute for files.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Save mtimes from a directory to a file.
    Save {
        /// The location to a file to save the current mtimes to.
        #[arg(short, long, default_value = "mtimes.json")]
        file: PathBuf,

        /// The location to recursively scan for files.
        target_dir: PathBuf,

        /// Regex patterns to skip.
        #[arg(short, long, num_args = 0..)]
        ignore: Vec<String>,

        /// Whether to be verbose.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
    /// Restore mtimes from a file.
    Restore {
        /// The location to a file to restore previous mtimes from.
        #[arg(short, long, default_value = "mtimes.json")]
        file: PathBuf,

        /// The location to recursively restore mtimes to.
        target_dir: PathBuf,

        /// Whether to be verbose.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,

        /// Whether to ignore matching hashes when restoring mtimes. Defaults to false.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        ignore_hash: bool,
    },
}
