#![allow(unused)]
#![allow(non_snake_case)]

pub mod sensors;
use std::env::args;
use std::ffi::OsString;
use std::io::{BufRead, BufReader, BufWriter, Read, StdoutLock};
use std::path;
use std::path::PathBuf;
use std::fs::File;
use clap::Parser;
use clap::Arg;
use anyhow::{Context, Result};
use std::io::{self, Write};
use crate::sensors::get_disk_usage;


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
}

#[derive(Parser)]
#[command(author, version, about, long_about= None)]
struct Commands {
    #[arg(short = 'd', long = "disks")]
    disks: bool,
    #[arg(short = 't', long = "temps")]
    temps: Option<String>,
    #[arg(short = 'r', long = "ram")]
    ram: Option<String>,
}
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Commands::parse();
    let stdout = io::stdout();
    let mut handler = io::BufWriter::new(stdout.lock());
    if args.disks {
        println!("disks was selected");
        get_disk_usage();
    }
    if let Some(temp) = args.temps.as_deref() {
        println!("temps was selected");
    }
    if let Some(ram) = args.ram.as_deref() {
        println!("ram was selected");
    }
    Ok(())
}



/*
let args = Cli::parse();
    let stdout = io::stdout();
    let pb = indicatif::ProgressBar::new(100);
    let mut handler = io::BufWriter::new(stdout.lock());

    let f = File::open(&args.path)
        .with_context(|| format!("Error reading {:?}", args.path))?;

    let mut reader = BufReader::new(&f);

    let mut holder = String::new();
    let mut count: u8 = 0;
    for line in reader.lines() {
        let line_val = String::from(line.unwrap());
        if line_val.contains(&args.pattern) {
            writeln!(handler, "{}", line_val);
        }
        count += 1;
        pb.println(format!("[+] finished processing line #{}", count));
        pb.inc(1);
       std::thread::sleep(core::time::Duration::from_millis(250));
    }
    pb.finish_with_message("complete");
*/
