use std::{collections::BTreeMap, fs, path::PathBuf};

use anyhow::{bail, Result};
use filetime::FileTime;
use walkdir::WalkDir;

use crate::{file_entry::FileEntry, hash::hash_file};

/// Save the mtimes.
pub(crate) fn save_mtimes(
    mtime_file_path: PathBuf,
    target_dir: PathBuf,
    ignore: Vec<PathBuf>,
    verbose: bool,
) -> Result<()> {
    if !target_dir.exists() {
        bail!("The target directory {target_dir:?} does not exist.")
    }

    println!(
        "Scanning `{}` and saving mtimes to `{}`.",
        target_dir.to_string_lossy(),
        mtime_file_path.to_string_lossy()
    );

    let ignore_paths = ignore
        .into_iter()
        .filter_map(|f| f.canonicalize().ok())
        .collect::<Vec<_>>();

    let mut data = BTreeMap::new();

    for entry in WalkDir::new(target_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| match e.path().canonicalize() {
            Ok(path) => !ignore_paths
                .iter()
                .any(|ignore_path| path.starts_with(ignore_path)),
            Err(_) => false,
        })
        .filter_map(|entry| entry.ok())
        .filter(|entry| !entry.path_is_symlink())
    {
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(err) => {
                if verbose {
                    eprintln!(
                        "Unable to get metadata for `{}` due to {:?}",
                        path.to_string_lossy(),
                        err
                    );
                }

                continue;
            }
        };

        let path_name = path.as_os_str().to_string_lossy().to_string();
        let mtime = FileTime::from_last_modification_time(&metadata).unix_seconds();
        let Ok(hash) = hash_file(&path) else {
            if verbose {
                eprintln!(
                    "Unable to get hash for `{}`, skipping.",
                    path.to_string_lossy(),
                );
            }

            continue;
        };

        let entry = FileEntry { mtime, hash };
        data.insert(path_name, entry);
    }

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(mtime_file_path, json)?;

    println!("Save complete, saved {} mtimes.", data.len());

    Ok(())
}
