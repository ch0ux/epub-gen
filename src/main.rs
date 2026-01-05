mod cli;
mod compress;
mod epub;
mod extract;
mod pipeline;

use crate::cli::Args;
use clap::Parser;
use std::{path::Path, process};

type CusResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    let args = Args::parse();

    if let Err(e) = pipeline::run_pipeline(
        &Path::new(&args.input),
        &Path::new(&args.output),
        args.quality,
        args.preserve_order,
    ) {
        println!("Error:{}", e);
        process::exit(1)
    }
}
