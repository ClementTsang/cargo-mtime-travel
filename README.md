# mtime-travel

[<img src="https://img.shields.io/crates/v/mtime-travel.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/mtime-travel)

A small tool to save and restore the mtime attribute for files.

This can be useful for things like avoiding Rust rebuilds if the file contents didn't change, but the mtimes did, as Rust
will rebuild based on mtimes (see <https://github.com/rust-lang/cargo/issues/6529>). A example where rebuilding like this
is undesirable is pulling a project via git in CI, as that will alter the mtime values and therefore normally trigger
a rebuild, even if you may already have cached the build artifacts from a prior CI run.

## Installation

You can install it via `cargo`:

```bash
cargo install --locked mtime-travel
```

## Usage

### Saving

```bash
Usage: mtime-travel save [OPTIONS] <TARGET_DIR>

Arguments:
  <TARGET_DIR>  The location to recursively scan for files

Options:
  -m, --mtime-file <MTIME_FILE>  The location to a file to save the current mtimes to [default: mtimes.json]
  -i, --ignore <IGNORE>          Regex patterns to skip
  -v, --verbose                  Whether to be verbose
  -h, --help                     Print help
```

To save the current directory to a file called `mtimes.json`:

```bash
mtime-travel save ./
```

To ignore certain regexes:

```bash
mtime-travel save --ignore ".*foo.*" ./
```

To save to another location:

```bash
mtime-travel save --mtime-file <MY_MTIME_FILE_PATH> ./
```

This will output a `.json` file with the files' hashes and mtime value.

### Restoring

```bash
Usage: mtime-travel restore [OPTIONS] <TARGET_DIR>

Arguments:
  <TARGET_DIR>  The location to recursively restore mtimes to

Options:
  -m, --mtime-file <MTIME_FILE>  The location to a file to restore previous mtimes from [default: mtimes.json]
  -v, --verbose                  Whether to be verbose
  -i, --ignore-hash              Whether to ignore hashes matching. Defaults to false
  -h, --help                     Print help
```

To restore the current directory's files given a file called `mtimes.json` in the same directory _if the file hashes match_:

```bash
mtime-travel restore ./
```

To ignore file hashes:

```bash
mtime-travel restore --ignore-hash ./
```

To use a different location for the saved mtime data:

```bash
mtime-travel restore --mtime-file <MY_MTIME_FILE_PATH> ./
```
