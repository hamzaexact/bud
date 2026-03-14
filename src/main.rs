#![allow(warnings)]
use rustyline::{error::ReadlineError};
use rustyline::DefaultEditor;

use std::path::{Path, PathBuf};

use clap::Parser;

use crate::input::read_command;

mod create;
mod input;
mod node;

use crate::create::Builder;

#[derive(Parser, Debug)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    let args = Args::parse();
    let mut absolute_path = std::fs::canonicalize(&args.path).expect("Failed to resolve path");
    let folder_name = absolute_path.file_name().unwrap().to_str().unwrap();
    let input = read_command(&mut rl, folder_name.to_string())?;
    let mut builder =   Builder::init(&*input);
    builder.create(absolute_path);

    Ok(())
}
