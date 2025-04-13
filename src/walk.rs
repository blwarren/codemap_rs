use crate::filter::is_excluded;
use ignore::gitignore::Gitignore;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn walk_filtered<'a>(
    root: &'a Path,
    gitignore: &'a Gitignore,
) -> impl Iterator<Item = DirEntry> + 'a {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(move |entry| !is_excluded(entry, root, gitignore))
}
