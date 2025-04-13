use codemap_rs::filter::{build_gitignore, is_excluded};
use ignore::gitignore::Gitignore;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use tempfile::tempdir;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// Helper to find a specific file entry in WalkDir
fn find_entry<'a>(root: &Path, name: &str) -> DirEntry {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .find(|e| e.path().file_name().map_or(false, |f| f == name))
        .expect("File not found")
}

#[test]
fn test_gitignore_excludes_target() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create `.gitignore`
    let mut ignore_file = File::create(root.join(".gitignore")).unwrap();
    writeln!(ignore_file, "/target").unwrap();

    // Create a file that should be ignored
    let ignored_dir = root.join("target");
    create_dir_all(&ignored_dir).unwrap();
    File::create(ignored_dir.join("ignored.txt")).unwrap().write_all(b"test").unwrap();

    // Create a file that should not be ignored
    let included_file = root.join("src.txt");
    File::create(&included_file).unwrap().write_all(b"keep me").unwrap();

    let gitignore: Gitignore = build_gitignore(root);

    let ignored_entry = find_entry(root, "ignored.txt");
    let included_entry = find_entry(root, "src.txt");

    assert!(
        is_excluded(&ignored_entry, root, &gitignore),
        "target/ignored.txt should be excluded"
    );
    assert!(
        !is_excluded(&included_entry, root, &gitignore),
        "src.txt should not be excluded"
    );
}
