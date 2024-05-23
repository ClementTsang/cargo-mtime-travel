use std::{collections::BTreeMap, fs, path::PathBuf};

use anyhow::{bail, Result};
use filetime::FileTime;
use regex::Regex;
use walkdir::WalkDir;

use crate::{file_entry::FileEntry, hash::hash_file};

/// Save the mtimes.
pub(crate) fn save_mtimes(
    mtime_file_path: PathBuf,
    target_dir: PathBuf,
    ignore: Vec<String>,
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

    let ignore_regexes = ignore
        .into_iter()
        .filter_map(|f| match Regex::new(&f) {
            Ok(regex) => Some(regex),
            Err(err) => {
                if verbose {
                    println!("the following regex is invalid and will be ignored: {f} - {err}");
                }
                None
            }
        })
        .collect::<Vec<_>>();

    if verbose {
        println!("Using the following regexes: {ignore_regexes:?}");
    }

    let mut data = BTreeMap::new();

    for path in WalkDir::new(target_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| {
            let path = e.ok().and_then(|e| e.path().canonicalize().ok());
            match path {
                Some(path) => {
                    let s = path.to_string_lossy();
                    if !ignore_regexes.iter().any(|re| re.is_match(&s)) {
                        Some(path)
                    } else {
                        None
                    }
                }
                None => None,
            }
        })
        .filter(|entry| !entry.is_symlink())
    {
        if path.is_dir() {
            continue;
        }

        let metadata = match fs::metadata(&path) {
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
        let (mtime, mtime_nano) = {
            let file_time = FileTime::from_last_modification_time(&metadata);
            (file_time.unix_seconds(), file_time.nanoseconds())
        };
        let Ok(hash) = hash_file(&path) else {
            if verbose {
                eprintln!(
                    "Unable to get hash for `{}`, skipping.",
                    path.to_string_lossy(),
                );
            }

            continue;
        };

        let entry = FileEntry {
            mtime,
            mtime_nano,
            hash,
        };
        data.insert(path_name, entry);
    }

    let json = serde_json::to_string_pretty(&data)?;
    fs::write(mtime_file_path, json)?;

    println!("Save complete, saved {} mtimes.", data.len());

    Ok(())
}
