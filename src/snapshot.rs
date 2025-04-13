use crate::walk::walk_filtered;
use ignore::gitignore::Gitignore;
use std::{collections::HashMap, fs, io::Write, path::Path};

const MAX_SIZE: u64 = 102_400;

pub fn print_header(writer: &mut impl Write) {
    writeln!(
        writer,
        "Directory and File Snapshot - {}",
        chrono::Local::now()
    )
    .unwrap();
    writeln!(
        writer,
        "======================================================\n"
    )
    .unwrap();
}

pub fn print_file_type_summary(writer: &mut impl Write, root: &Path, gitignore: &Gitignore) {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for entry in walk_filtered(root, gitignore) {
        let path = entry.path();
        if path.is_file() {
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

pub fn print_working_directory(writer: &mut impl Write, path: &Path) {
    writeln!(writer, "Working Directory Full Path:\n{}\n", path.display()).unwrap();
}

pub fn print_directory_tree(writer: &mut impl Write, root: &Path, gitignore: &Gitignore) {
    writeln!(
        writer,
        "Directory Structure Diagram (excluding entries in .gitignore):"
    )
    .unwrap();

    for entry in walk_filtered(root, gitignore) {
        let path = entry.path();
        if let Ok(rel_path) = path.strip_prefix(root) {
            writeln!(writer, "{}", rel_path.display()).unwrap();
        }
    }

    writeln!(writer).unwrap();
}

pub fn process_files(
    writer: &mut impl Write,
    skipped: &mut Vec<String>,
    root: &Path,
    output_path: &Path,
    gitignore: &Gitignore,
) {
    writeln!(
        writer,
        "======================================================\nIncluded Files (UTF-8 text, <= {} bytes):\n",
        MAX_SIZE
    )
    .unwrap();

    for entry in walk_filtered(root, gitignore) {
        let path = entry.path();
        if path == output_path || !path.is_file() {
            continue;
        }

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
                writeln!(
                    writer,
                    "======================================================"
                )
                .unwrap();
                writeln!(writer, "File: {}", path.display()).unwrap();
                writeln!(writer, "Size: {} bytes", metadata.len()).unwrap();
                writeln!(
                    writer,
                    "======================================================"
                )
                .unwrap();
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

pub fn print_skipped_summary(writer: &mut impl Write, skipped: &[String]) {
    if !skipped.is_empty() {
        writeln!(
            writer,
            "======================================================"
        )
        .unwrap();
        writeln!(writer, "Skipped Files:\n").unwrap();
        for line in skipped {
            writeln!(writer, "{}", line).unwrap();
        }
    }
}
