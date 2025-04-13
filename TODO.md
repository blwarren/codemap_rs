# TODO: codemap_rs Development Roadmap

A list of planned improvements and feature ideas for `codemap_rs`.

---

## ✅ Features (Complete)

- Traverse directory tree recursively
- Skip paths based on `.gitignore`
- Display file type summary
- Show directory structure
- Include UTF-8 text file contents ≤ 100 KB
- Skip binary and oversized files
- Output results to `directory_snapshot.txt`
- Integration tests for `.gitignore` filtering

---

## 🧭 CLI Enhancements

- [ ] Integrate `clap` for argument parsing
- [ ] `--output <FILE>`: specify custom output path
- [ ] `--max-size <BYTES>`: customize max size for file content preview
- [ ] `--no-content`: skip showing file contents entirely
- [ ] `--include-binary`: include binary files in the output
- [ ] `--summary-only`: print summary and structure, skip files
- [ ] `--version` and `--help` flags

---

## 🧪 Testing and Coverage

- [ ] Add unit tests for `snapshot.rs`, especially `process_files`
- [ ] Add regression tests for skipped files (binary, size, permissions)
- [ ] Add test for symlink resolution behavior
- [ ] Add test to ensure `.git` is always excluded, even if not in `.gitignore`

---

## 📄 File Insight Features

- [ ] Optionally display file metadata: size, modified time, permissions
- [ ] Count and display number of lines in text files
- [ ] Summarize known license types if detected (`MIT`, `Apache`, etc.)

---

## 📂 Output Formats

- [ ] Support `--format json` for machine-readable output
- [ ] Option for Markdown output
- [ ] Add `--summary-only` mode that omits included file content

---

## 🧰 Usability Improvements

- [ ] Show progress bar for long traversals (via `indicatif`)
- [ ] Use `tracing` or `env_logger` for verbose/debug mode
- [ ] Add dry-run mode to validate `.gitignore` filtering without generating output

---

## 🔮 Future Ideas

- [ ] Read optional `codemap.toml` config file for defaults (ignored files, max size, etc.)
- [ ] Add plugin system for processing matched files in custom ways
- [ ] Publish binaries via GitHub Releases
- [ ] Create a Homebrew formula for macOS installs
- [ ] Explore WASM version for web-based directory introspection

---

## 🧾 Miscellaneous

- [ ] Restore and refine `inst_local.sh` for local install helper
- [ ] Update README with any CLI changes
- [ ] Create a changelog for future releases

---