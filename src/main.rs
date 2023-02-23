use std::collections::HashMap;
use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use clap::{Parser, ValueHint};

use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::ParallelIterator;

use twox_hash::Xxh3Hash128;

use walkdir::{DirEntry, WalkDir};

struct FileEntry {
    pub path: PathBuf,
    pub hash: u64
}

impl FileEntry {
    pub fn from_dir_entry(entry: &DirEntry) -> Result<Self, std::io::Error> {
        let mut file = File::open(entry.path())?;
        let mut digest = Xxh3Hash128::default();
        let mut seek_distance = 1;

        loop {
            let mut buffer = [0; 4096];

            match file.read(&mut buffer)? {
                0 => break,
                n => digest.write(&buffer[..n])
            }

            file.seek(SeekFrom::Current(seek_distance))?;
            seek_distance >>= 1;
        }

        Ok(FileEntry {
            path: entry.path().to_path_buf(),
            hash: digest.finish()
        })
    }
}

fn file_hash(entry: &DirEntry) -> Option<FileEntry> {
    match FileEntry::from_dir_entry(&entry) {
        Ok(entry) => Some(entry),
        Err(e) => {
            eprintln!("\"{}\": {}", entry.path().display(), e);
            None
        }
    }
}

fn dir_file_hashes(directory: &Path) -> HashMap<u64, Vec<FileEntry>> {
    let paths: Vec<DirEntry> = WalkDir::new(&directory)
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let entries: Vec<FileEntry> = paths.par_iter()
        .filter(|entry| entry.file_type().is_file())
        .filter_map(file_hash)
        .collect();

    let mut hashes = HashMap::new();

    for entry in entries {
        hashes.entry(entry.hash)
            .or_insert_with(Vec::new)
            .push(entry)
    }

    hashes
}

fn find_same_file(directory: &Path) -> HashMap<u64, Vec<FileEntry>> {
    dir_file_hashes(directory)
        .into_iter()
        .filter(|(_hash, entries)| entries.len() > 1)
        .collect()
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub directory: PathBuf,
}

fn main() {
    let args: Args = Args::parse();

    for (k, v) in find_same_file(&args.directory) {
        println!("{:X}:", k);

        for entry in v {
            println!("\t- {}", entry.path.display());
        }

        print!("\n\n");
    }
}