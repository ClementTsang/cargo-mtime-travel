use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct FileEntry {
    pub mtime: i64,
    pub mtime_nano: u32,
    pub hash: String,
}
