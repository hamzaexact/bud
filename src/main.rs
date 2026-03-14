#![allow(warnings)]
mod create;
mod input;
use crate::create::Builder;
use crate::input::read_command;
use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let path = std::fs::canonicalize(&args.path)?;
    let folder_name = path.file_name().unwrap().to_str().unwrap();
    let input = read_command(folder_name)?;
    Builder::init(&input).create(path);
    Ok(())
}
