use std::{collections::BTreeMap, fs, path::{Path, PathBuf}};
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FileIndex(BTreeMap<String, Vec<PathBuf>>);

fn is_skippable(path: &Path) -> bool {
    let skippable = ["/proc", "/sys", "/dev", "/run", "/snap", "/var/lib"];
    skippable.iter().any(|prefix| path.starts_with(prefix))
}

fn main() {
    let mut index: BTreeMap<String, Vec<PathBuf>> = BTreeMap::new();

    let roots = if cfg!(windows) {
        (b'C'..=b'Z')
            .filter_map(|letter| {
                let drive = format!("{}:\\", letter as char);
                let path = PathBuf::from(&drive);
                if path.exists() {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<PathBuf>>()
    } else {
        vec![PathBuf::from("/")]
    };

    for root in roots {
        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !is_skippable(e.path()))
            .filter(|e| e.file_type().is_file() || e.file_type().is_dir())
        {
            if let Some(name) = entry.file_name().to_str() {
                index.entry(name.to_string())
                     .or_default()
                     .push(entry.path().to_path_buf());
            }
        }
    }

    let serialized = serde_json::to_string(&FileIndex(index)).unwrap();
    fs::write("file_index.json", serialized).unwrap();

    println!("Index built and saved to 'file_index.json'.");
}
