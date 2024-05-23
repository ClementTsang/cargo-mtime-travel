# mtime-travel

[<img src="https://img.shields.io/crates/v/mtime-travel.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/mtime-travel)

A small tool to save and restore the mtime attribute for files. When saving, it will record the file hashes, and when restoring,
it will only restore the mtime if the file hash matches. File hashing is done using SHA-256.

This can be useful for things like avoiding Rust rebuilds if the file contents didn't change, but the mtimes did, as Rust
will rebuild based on mtimes (see <https://github.com/rust-lang/cargo/issues/6529>). A example where rebuilding like this
is undesirable is pulling a project via git in CI, as that will alter the mtime values and therefore normally trigger
a rebuild, even if you may already have cached the build artifacts from a prior CI run.

## Installation

You can install mtime-travel via `cargo`:

```bash
cargo install --locked mtime-travel
```

## Usage

### Saving

```shell
Usage: mtime-travel save [OPTIONS] <TARGET_DIR>

Arguments:
  <TARGET_DIR>  The location to recursively scan for files

Options:
  -f, --file <FILE>           The location to a file to save the current mtimes to [default: mtimes.json]
  -i, --ignore [<IGNORE>...]  Regex patterns for which files to skip; multiple regex strings can be passed. Note the file paths checked are the absolute paths
  -v, --verbose               Enable verbose output
  -h, --help                  Print help
```

To save the current directory to a file called `mtimes.json`:

```shell
mtime-travel save ./
```

To ignore certain regexes:

```shell
mtime-travel save --ignore ".*foo.*" ./
```

To save to another location:

```shell
mtime-travel save --file <MTIME_FILE_PATH> ./
```

This will output a `.json` file with the files' hashes and mtime value.

### Restoring

```shell
Usage: mtime-travel restore [OPTIONS] <TARGET_DIR>

Arguments:
  <TARGET_DIR>  The location to recursively restore mtimes to

Options:
  -f, --file <FILE>  The location to a file to restore previous mtimes from [default: mtimes.json]
  -v, --verbose      Enable verbose output
  -i, --ignore-hash  Restore mtime for a file even if the hash does not match
  -h, --help         Print help
```

To restore the current directory's files given a file called `mtimes.json` in the same directory _if the file hashes match_:

```shell
mtime-travel restore ./
```

To ignore file hashes:

```shell
mtime-travel restore --ignore-hash ./
```

To restore from a file at a different path:

```shell
mtime-travel restore --file <MTIME_FILE_PATH> ./
```
