use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser)]
pub struct Arguments {
    #[clap(short, long, value_hint = ValueHint::DirPath)]
    pub directory: PathBuf
}