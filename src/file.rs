use std::fs::File;
use std::hash::Hasher;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use twox_hash::Xxh3Hash128;
use twox_hash::xxh3::HasherExt;

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct FileStats {
    pub hash: u128,
    pub size: u64
}

#[derive(PartialEq, Eq)]
pub struct FileEntry {
    pub path: PathBuf,
    pub stats: FileStats
}

impl FileEntry {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        Ok(FileEntry {
            path: path.as_ref().to_path_buf(),
            stats: FileStats {
                hash: hash_file(&path)?,
                size: std::fs::metadata(&path)?.len()
            }
        })
    }
}

fn hash_file<P: AsRef<Path>>(path: P) -> Result<u128, std::io::Error> {
    let mut file = File::open(path)?;
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

    Ok(digest.finish_ext())
}