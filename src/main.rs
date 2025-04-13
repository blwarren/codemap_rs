use codemap_rs::filter::build_gitignore;
use codemap_rs::snapshot::{
    print_directory_tree, print_file_type_summary, print_header, print_skipped_summary,
    print_working_directory, process_files,
};

use std::{env, fs::File, io::BufWriter, path::PathBuf};

const OUTPUT_FILE: &str = "directory_snapshot.txt";

fn main() {
    let target_dir = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let root = PathBuf::from(&target_dir).canonicalize().unwrap();
    let output_path = root.join(OUTPUT_FILE);
    let gitignore = build_gitignore(&root);

    let file = File::create(&output_path).expect("Could not create output file");
    let mut writer = BufWriter::new(file);

    let mut skipped: Vec<String> = Vec::new();

    print_header(&mut writer);
    print_file_type_summary(&mut writer, &root, &gitignore);
    print_working_directory(&mut writer, &root);
    print_directory_tree(&mut writer, &root, &gitignore);
    process_files(&mut writer, &mut skipped, &root, &output_path, &gitignore);
    print_skipped_summary(&mut writer, &skipped);
}
