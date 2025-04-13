use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::Path;
use walkdir::DirEntry;

pub fn build_gitignore(root: &Path) -> Gitignore {
    let mut builder = GitignoreBuilder::new(root);
    builder.add(root.join(".gitignore"));
    builder.build().expect("Failed to build gitignore")
}

pub fn is_excluded(entry: &DirEntry, root: &Path, gitignore: &Gitignore) -> bool {
    let path = entry.path();
    let relative = path.strip_prefix(root).unwrap_or(path);

    if relative.components().any(|c| c.as_os_str() == ".git") {
        return true;
    }

    gitignore
        .matched_path_or_any_parents(relative, path.is_dir())
        .is_ignore()
}
