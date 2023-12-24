use std::{
    collections::BTreeMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::Result;
use filetime::FileTime;

use crate::{file_entry::FileEntry, hash::hash_file};

/// Restore the mtimes.
pub(crate) fn restore_mtimes(
    mtime_file_path: PathBuf,
    target_dir: PathBuf,
    verbose: bool,
) -> Result<()> {
    let data: BTreeMap<String, FileEntry> = {
        let mtime_file = File::open(&mtime_file_path)?;
        serde_json::from_reader(mtime_file)?
    };

    println!(
        "Restoring from `{}` to `{}`",
        mtime_file_path.to_string_lossy(),
        target_dir.to_string_lossy()
    );

    let mut num_restored = 0;

    for (file, entry) in data {
        let path = Path::new(&file);

        if !path.exists() {
            continue;
        }

        let Ok(hash) = hash_file(path) else {
            if verbose {
                eprintln!(
                    "Unable to get hash for `{}`, skipping.",
                    path.to_string_lossy(),
                );
            }

            continue;
        };

        if hash == entry.hash {
            let metadata = match fs::metadata(path) {
                Ok(metadata) => metadata,
                Err(err) => {
                    if verbose {
                        eprintln!(
                            "Unable to get metadata for `{}` due to {:?}.",
                            path.to_string_lossy(),
                            err
                        );
                    }

                    continue;
                }
            };

            let mtime = FileTime::from_last_modification_time(&metadata);
            if let Err(err) = filetime::set_file_mtime(path, mtime) {
                if verbose {
                    eprintln!("Unable to set mtime for `{file}` due to {err:?}.");
                }

                continue;
            }

            num_restored += 1;
        } else if verbose {
            println!(
                "Skipping restore for {} due to hash mismatch (`{}` vs `{}`).",
                file, hash, entry.hash
            );
        }
    }

    println!("Restore complete, restored {num_restored} mtimes.");

    Ok(())
}
