#![allow(warnings)]
#![allow(unused_imports)]
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use std::path::{PathBuf, Path};

use clap::{Parser};

mod node;
mod create;
mod input;

#[derive(Parser, Debug)]  
struct Args {
    #[arg(default_value = ".")]

    path: PathBuf
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let command = input::read_command(&mut rl, "HAMZA".to_string());

}

