use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct FileEntry {
    pub mtime: i64,
    pub hash: String,
}
