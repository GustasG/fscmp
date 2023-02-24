use std::collections::HashMap;
use std::path::Path;

use clap::Parser;

use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelBridge;
use rayon::prelude::ParallelIterator;

use walkdir::{DirEntry, WalkDir, Error};

mod args;
mod file;

use args::Arguments;
use file::FileEntry;

use crate::file::FileStats;

fn hash_directory<P: AsRef<Path>>(root: P) -> Vec<FileEntry> {
    fn run(entry: Result<DirEntry, Error>) -> Option<FileEntry> {
        match entry {
            Ok(value) => {
                match FileEntry::from_path(value.path()) {
                    Ok(entry) => Some(entry),
                    Err(e) => {
                        eprintln!("Cannot open \"{}\": {}", value.path().display(), e);
                        None
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    WalkDir::new(root)
        .into_iter()
        .par_bridge()
        .filter_map(run)
        .collect()
}

fn find_duplicates<P: AsRef<Path>>(root: P) -> HashMap<FileStats, Vec<FileEntry>> {
    let entries = hash_directory(root);
    let mut hashes = HashMap::with_capacity(entries.len());

    for entry in entries {
        hashes.entry(entry.stats.clone())
            .or_insert_with(Vec::new)
            .push(entry);
    }

    hashes.into_par_iter()
        .filter(|(_, value)| value.len() > 1)
        .collect()
}

fn main() {
    let args = Arguments::parse();
    let hashes = find_duplicates(args.directory);

    println!("Dupliace files:\n");

    for (i, (_, v)) in hashes.iter().enumerate() {
        println!("Pair ([{}/{}]):", i + 1, hashes.len());

        for entry in v {
            println!("\t- {}", entry.path.display());
        }

        println!("\n\n");
    }
}