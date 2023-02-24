use std::collections::HashMap;

use clap::Parser;

use rayon::prelude::IntoParallelIterator;
use rayon::prelude::ParallelBridge;
use rayon::prelude::ParallelIterator;

use walkdir::{DirEntry, WalkDir};

mod args;
mod file;

use args::Arguments;
use file::FileEntry;

use crate::file::FileStats;

fn hash_directory(args: &Arguments) -> Vec<FileEntry> {
    fn run(entry: DirEntry) -> Option<FileEntry> {
        match FileEntry::from_path(entry.path()) {
            Ok(value) => Some(value),
            Err(e) => {
                eprintln!("Cannot open \"{}\": {}", entry.path().display(), e);
                None
            }
        }
    }

    WalkDir::new(&args.directory)
        .into_iter()
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(run)
        .collect()
}

fn find_duplicates(args: &Arguments) -> HashMap<FileStats, Vec<FileEntry>> {
    let entries = hash_directory(&args);
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
    let hashes = find_duplicates(&args);

    println!("Duplicate files:\n");

    for (i, (_, v)) in hashes.iter().enumerate() {
        println!("Group ([{}/{}]):", i + 1, hashes.len());

        for entry in v {
            println!("\t- {}", entry.path.display());
        }

        println!("\n\n");
    }
}