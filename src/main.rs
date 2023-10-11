#![allow(unused)]
#![allow(non_snake_case)]

pub mod sensors;
use std::env::args;
use std::io::{BufRead, BufReader, Read};
use std::path;
use std::path::PathBuf;
use std::fs::File;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    let f = File::open(&args.path)?;
    let mut reader = BufReader::new(&f);

    let mut holder = String::new();
    for line in reader.lines() {
        let line_val = String::from(line.unwrap());
        if line_val.contains(&args.pattern) {
            println!("{}", line_val);
        }
    }
    Ok(())
}

