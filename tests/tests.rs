use std::{
    env,
    fs::File,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
    process::Command,
};

use filetime::FileTime;

fn binary() -> Command {
    let exe = env!("CARGO_BIN_EXE_mtime-travel");
    Command::new(exe)
}

fn save(path: &str, ignore: Vec<&str>, file: &str) {
    if !ignore.is_empty() {
        assert!(binary()
            .current_dir(path)
            .arg("save")
            .arg("--ignore")
            .args(ignore)
            .arg("--verbose")
            .arg("--file")
            .arg(file)
            .arg("./")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    } else {
        assert!(binary()
            .current_dir(path)
            .arg("save")
            .arg("--verbose")
            .arg("--file")
            .arg(file)
            .arg("./")
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .success());
    }
}

fn restore(path: &str, file: &str) {
    assert!(binary()
        .current_dir(path)
        .arg("restore")
        .arg("--verbose")
        .arg("--file")
        .arg(file)
        .arg("./")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());
}

fn assert_num_keys(path: &Path, num_keys: usize) {
    let file = File::open(path).unwrap();
    let mapping: serde_json::Map<String, serde_json::Value> =
        serde_json::from_reader(file).unwrap();

    assert_eq!(mapping.len(), num_keys);
}

fn save_and_restore(path: &str, ignore: Vec<&str>, file: &str, num_keys: Option<usize>) {
    // Delete any leftovers from before if they exist.
    let generated_file = PathBuf::from(format!("{path}/{file}"));
    if generated_file.exists() {
        let _ = std::fs::remove_file(&generated_file);
    }

    save(path, ignore, file);
    assert!(generated_file.exists());
    if let Some(num_keys) = num_keys {
        assert_num_keys(&generated_file, num_keys);
    }

    restore(path, file);

    // Cleanup
    let _ = std::fs::remove_file(&generated_file);
}

#[test]
fn basic() {
    save_and_restore("./tests/test_dir", vec![".*\\.json"], "basic.json", None);
}

#[test]
fn ignore() {
    save_and_restore(
        "./tests/test_dir",
        vec![".*\\.json"],
        "ignore.json",
        Some(4),
    );
}

#[test]
fn multiple_ignore() {
    save_and_restore(
        "./tests/test_dir",
        vec![".*\\.json", ".*a\\.txt"],
        "multiple_ignore.json",
        Some(2),
    );
}

#[test]
fn test_nested() {
    save_and_restore(
        "./tests/test_dir/nested",
        vec![".*\\.json", ".*a\\.txt"],
        "nested.json",
        Some(1),
    );
}

#[test]
fn check_values() {
    let path = "./tests/test_dir_2";
    let file = "check_values.json";
    let tested_file = PathBuf::from(format!("{path}/a.txt"))
        .canonicalize()
        .unwrap();

    assert!(tested_file.exists());

    // Delete any leftovers from before if they exist.
    let generated_file = PathBuf::from(format!("{path}/{file}"));
    if generated_file.exists() {
        let _ = std::fs::remove_file(&generated_file);
    }

    save(path, vec![], file);
    assert!(generated_file.exists());

    // Check the mtime of the generated file.
    let mapping: serde_json::Map<String, serde_json::Value> =
        serde_json::from_reader(File::open(&generated_file).unwrap()).unwrap();
    let saved_mtime = (&mapping)[tested_file.to_string_lossy().as_ref()]
        .as_object()
        .unwrap()["mtime"]
        .as_i64()
        .unwrap();
    assert_eq!(saved_mtime, tested_file.metadata().unwrap().mtime());

    // Now change the mtime.
    filetime::set_file_mtime(&tested_file, FileTime::now()).unwrap();
    assert_ne!(saved_mtime, tested_file.metadata().unwrap().mtime());

    // Then restore it.
    restore(path, file);
    assert_eq!(saved_mtime, tested_file.metadata().unwrap().mtime());

    // Cleanup
    let _ = std::fs::remove_file(&generated_file);
}

#[test]
fn test_different_directory() {
    let initial_path = "./tests/test_dir/nested";
    let second_path = "./tests/test_dir";
    let file = "different_directory.json";

    let nested_files = [
        PathBuf::from(format!("{initial_path}/nested_a.txt"))
            .canonicalize()
            .unwrap(),
        PathBuf::from(format!("{initial_path}/nested_b.txt"))
            .canonicalize()
            .unwrap(),
    ];

    // Delete any leftovers from before if they exist.
    let generated_file = PathBuf::from(format!("{initial_path}/{file}"));
    if generated_file.exists() {
        let _ = std::fs::remove_file(&generated_file);
    }

    save(initial_path, vec![".*\\.json"], file);
    assert!(generated_file.exists());
    let generated_file = generated_file.canonicalize().unwrap();
    assert_num_keys(&generated_file, 2);

    // Check the mtime of the generated file.
    let mapping: serde_json::Map<String, serde_json::Value> =
        serde_json::from_reader(File::open(&generated_file).unwrap()).unwrap();

    for f in &nested_files {
        let saved_mtime = (&mapping)[f.to_string_lossy().as_ref()]
            .as_object()
            .unwrap()["mtime"]
            .as_i64()
            .unwrap();
        assert_eq!(saved_mtime, f.metadata().unwrap().mtime());

        // Now change the mtime.
        filetime::set_file_mtime(f, FileTime::now()).unwrap();
        assert_ne!(saved_mtime, f.metadata().unwrap().mtime());
    }

    assert!(binary()
        .current_dir(second_path)
        .arg("restore")
        .arg("--verbose")
        .arg("--file")
        .arg(generated_file.to_string_lossy().as_ref())
        .arg("./")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());

    // Now verify they're back to normal.
    for f in &nested_files {
        let saved_mtime = (&mapping)[f.to_string_lossy().as_ref()]
            .as_object()
            .unwrap()["mtime"]
            .as_i64()
            .unwrap();
        assert_eq!(saved_mtime, f.metadata().unwrap().mtime());
    }

    // Cleanup
    let _ = std::fs::remove_file(&generated_file);
}
