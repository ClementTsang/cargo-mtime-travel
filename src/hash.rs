use std::{
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Result;
use data_encoding::HEXUPPER;
use sha2::{Digest, Sha256};

fn hash_sha256<R: Read>(mut reader: R) -> Result<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let digest = hasher.finalize();

    Ok(HEXUPPER.encode(&digest))
}

pub(crate) fn hash_file(path: &Path) -> Result<String> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let hash = hash_sha256(reader)?;

    Ok(hash)
}
