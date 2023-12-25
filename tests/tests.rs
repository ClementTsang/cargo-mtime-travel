use std::{env, process::Command};

fn binary() -> Command {
    let exe = env!("CARGO_BIN_EXE_cargo-mtime-travel");
    Command::new(exe)
}

fn save(path: &str, ignore: Option<&str>) {
    if let Some(ignore) = ignore {
        binary()
            .current_dir(path)
            .arg("save")
            .arg("--ignore")
            .arg(ignore)
            .arg("--verbose")
            .arg("./")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        binary()
            .current_dir(path)
            .arg("save")
            .arg("--verbose")
            .arg("./")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

fn restore(path: &str) {
    binary()
        .current_dir(path)
        .arg("restore")
        .arg("--verbose")
        .arg("./")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[test]
fn basic() {
    save("./tests/test_dir", None);
    restore("./tests/test_dir");
}

#[test]
fn ignore() {
    save("./tests/test_dir", Some("*.a.txt"));
    restore("./tests/test_dir");
}

// fn test_nested(){}

// fn check_values(){}
