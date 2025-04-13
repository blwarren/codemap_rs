# codemap_rs

`codemap_rs` is a fast and portable Rust CLI utility that generates a comprehensive snapshot of a directory tree. It excludes files and directories based on `.gitignore` rules and produces a human-readable report of the directory structure, file types, and included file contents.

---

## 📦 Features

- ✅ Traverses a directory recursively
- 🚫 Excludes paths matching `.gitignore` rules
- 📝 Generates a report showing:
  - File type summary by extension
  - Directory structure diagram
  - UTF-8 file contents for files ≤ 100 KB
- 📂 Skips binary or oversized files
- 🔍 Honors real Git behavior by always excluding `.git/`

---

## 🛠 Installation

Build and install to your local bin directory:

```bash
cargo build --release
cp target/release/codemap_rs ~/.local/bin/codemap
```

## 🚀 Usage

```bash
codemap [target_dir]
```

If `target_dir` is omitted, it defaults to the current directory.

The snapshot will be saved to `directory_snapshot.txt` in the target directory.

---

## 📁 Output Example

```
Directory and File Snapshot - 2025-04-13 18:29:54

Summary of File Types (by extension):
    6 rs
    1 lock
    1 toml

Working Directory Full Path:
/home/bobby/projects/codemap_rs

Directory Structure Diagram (excluding entries in .gitignore):
LICENSE
tests/filter_tests.rs
Cargo.toml
src/lib.rs
src/main.rs
...
```

---

## 🧪 Testing

Run all tests:

```bash
cargo test
```

Includes:
- Integration tests for `.gitignore` exclusions
- Test coverage for core path-filtering logic

---

## 📚 Project Structure

```
src/
├── filter.rs      # Handles .gitignore parsing and exclusion
├── walk.rs        # Directory traversal with filters applied
├── snapshot.rs    # Output formatting and report generation
├── lib.rs         # Public API for tests and main
└── main.rs        # CLI entry point
tests/
└── filter_tests.rs  # Integration tests for exclusion behavior
```

---

## 🧾 License

MIT License. See [LICENSE](LICENSE) for details.