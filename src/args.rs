use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// cargo-mtime-travel stores and restores the mtime attribute for files.
///
/// This can be helpful when trying to avoid rebuilding in Rust projects
/// when leveraging caching (see https://github.com/rust-lang/cargo/issues/6529
/// for more details).
#[derive(Parser)]
#[command(author, version, about, long_about(None))]
#[command(arg_required_else_help(true))]
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
        mtime_file: PathBuf,

        /// The location to recursively scan for files.
        target_dir: PathBuf,

        /// Files/directories to not scan. Supports globbing.
        #[arg(short, long)]
        ignore: Vec<PathBuf>,

        /// Whether to be verbose.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
    /// Restore mtimes from a file.
    Restore {
        /// The location to a file to restore previous mtimes from.
        #[arg(short, long, default_value = "mtimes.json")]
        mtime_file: PathBuf,

        /// The location to recursively restore mtimes to.
        target_dir: PathBuf,

        /// Whether to be verbose.
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
}
