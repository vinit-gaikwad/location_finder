use std::{collections::BTreeMap, fs, io::{self, Write}, path::PathBuf};
use serde::{Deserialize};

#[derive(Deserialize)]
struct FileIndex(BTreeMap<String, Vec<PathBuf>>);

fn main() {
    let json = fs::read_to_string("file_index.json")
        .expect("Index file not found. Run indexer first.");

    let index: FileIndex = serde_json::from_str(&json).unwrap();

    loop {
        print!("Enter filename to search (or 'exit'): ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let query = query.trim();

        if query == "exit" {
            break;
        }

        match index.0.get(query) {
            Some(paths) => {
                println!("Found {} result(s):", paths.len());
                for path in paths {
                    println!("{}", path.display());
                }
            }
            None => println!("No files named '{}'", query),
        }
    }
}
