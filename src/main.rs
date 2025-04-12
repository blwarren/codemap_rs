use chrono::Local;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

const MAX_SIZE: u64 = 102_400;
const OUTPUT_FILE: &str = "directory_snapshot.txt";
const EXCLUDED_DIRS: &[&str] = &[".git", ".venv", "__pycache__", ".github", ".pytest_cache", ".ruff_cache", "target"];

fn is_excluded(path: &Path, root: &Path) -> bool {
    EXCLUDED_DIRS.iter().any(|dir| path.starts_with(root.join(dir)))
}

fn print_header(writer: &mut impl Write) {
    writeln!(writer, "Directory and File Snapshot - {}", Local::now()).unwrap();
    writeln!(writer, "======================================================\n").unwrap();
}

fn print_file_type_summary(writer: &mut impl Write, root: &Path) {
    let mut counts = std::collections::HashMap::new();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() && !is_excluded(path, root) {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                *counts.entry(ext.to_lowercase()).or_insert(0) += 1;
            }
        }
    }

    writeln!(writer, "Summary of File Types (by extension):").unwrap();
    let mut count_vec: Vec<_> = counts.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    for (ext, count) in count_vec {
        writeln!(writer, "{:>5} {}", count, ext).unwrap();
    }
    writeln!(writer).unwrap();
}

fn print_working_directory(writer: &mut impl Write, path: &Path) {
    writeln!(writer, "Working Directory Full Path:\n{}\n", path.display()).unwrap();
}

fn print_directory_tree(writer: &mut impl Write, root: &Path) {
    writeln!(
        writer,
        "Directory Structure Diagram (excluding {}):",
        EXCLUDED_DIRS.join(", ")
    )
    .unwrap();

    if let Ok(output) = Command::new("tree")
        .arg("-a")
        .arg("-I")
        .arg(EXCLUDED_DIRS.join("|"))
        .arg(root)
        .output()
    {
        writer.write_all(&output.stdout).unwrap();
    } else {
        for entry in WalkDir::new(root) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if !is_excluded(path, root) {
                    writeln!(writer, "{}", path.strip_prefix(root).unwrap().display()).unwrap();
                }
            }
        }
    }
    writeln!(writer).unwrap();
}

fn process_files(writer: &mut impl Write, skipped: &mut Vec<String>, root: &Path, output_path: &Path) {
    writeln!(
        writer,
        "======================================================\nIncluded Files (UTF-8 text, <= {} bytes):\n",
        MAX_SIZE
    )
    .unwrap();

    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file()
            && !is_excluded(path, root)
            && path != output_path
        {
            let metadata = match fs::metadata(path) {
                Ok(meta) => meta,
                Err(_) => continue,
            };

            if metadata.len() > MAX_SIZE {
                skipped.push(format!(
                    "Skipped (too large >{}B): {} (Size: {} bytes)",
                    MAX_SIZE,
                    path.display(),
                    metadata.len()
                ));
                continue;
            }

            let content = match fs::read(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            match String::from_utf8(content) {
                Ok(text) => {
                    writeln!(writer, "======================================================").unwrap();
                    writeln!(writer, "File: {}", path.display()).unwrap();
                    writeln!(writer, "Size: {} bytes", metadata.len()).unwrap();
                    writeln!(writer, "======================================================").unwrap();
                    writeln!(writer, "{}\n", text).unwrap();
                }
                Err(_) => {
                    skipped.push(format!(
                        "Skipped (binary or invalid UTF-8): {}",
                        path.display()
                    ));
                }
            }
        }
    }
}

fn print_skipped_summary(writer: &mut impl Write, skipped: &[String]) {
    if !skipped.is_empty() {
        writeln!(writer, "======================================================").unwrap();
        writeln!(writer, "Skipped Files:\n").unwrap();
        for line in skipped {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}

fn main() {
    let target_dir = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let root = PathBuf::from(&target_dir).canonicalize().unwrap();
    let output_path = root.join(OUTPUT_FILE);

    let file = File::create(&output_path).expect("Could not create output file");
    let mut writer = BufWriter::new(file);

    let mut skipped: Vec<String> = Vec::new();

    print_header(&mut writer);
    print_file_type_summary(&mut writer, &root);
    print_working_directory(&mut writer, &root);
    print_directory_tree(&mut writer, &root);
    process_files(&mut writer, &mut skipped, &root, &output_path);
    print_skipped_summary(&mut writer, &skipped);
}
