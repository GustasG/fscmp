use std::path::PathBuf;

use clap::{Parser, ValueHint};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[clap(short, long, value_hint = ValueHint::DirPath, default_value_os_t = PathBuf::from("."))]
    pub directory: PathBuf
}