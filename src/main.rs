#![allow(dead_code)]
#![allow(unused_imports)]

use std::path::{PathBuf, Path};
use clap::{Parser};

mod node;
mod create;

#[derive(Parser, Debug)]  
struct Args {
    #[arg(default_value = ".")]

    path: PathBuf
}

fn main() {
    // let args = Args::parse();
    // let absolute_path = std::fs::canonicalize(&args.path)
    //     .expect("Failed to resolve path");
    // 
    // println!("Absolute path: {:?}", absolute_path);
}

