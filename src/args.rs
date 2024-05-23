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

        /// Regex patterns for which files to skip; multiple regex strings can be passed.
        /// Note the file paths checked are the absolute paths.
        #[arg(short, long, num_args = 0..)]
        ignore: Vec<String>,

        /// Enable verbose output.
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

        /// Enable verbose output.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,

        /// Restore mtime for a file even if the hash does not match.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        ignore_hash: bool,
    },
}
