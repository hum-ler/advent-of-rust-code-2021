use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser)]
struct Args {
    part: u8,

    #[arg(short = 'i', long)]
    input: Option<String>,
}

pub enum Part {
    Part1(String),
    Part2(String),
}

/// Gets the [Part] to execute.
pub fn get_part(default_input: &str) -> Result<Part> {
    let args = Args::parse();

    let path = args.input.unwrap_or(default_input.to_string());
    let input = trim_input(&read_to_string(path)?).to_string();

    match args.part {
        1 => Ok(Part::Part1(input)),
        2 => Ok(Part::Part2(input)),
        x => Err(anyhow!("Invalid part number: {}", x)),
    }
}

/// Trims newlines from the start and the end of the input string.
pub fn trim_input(input: &str) -> &str {
    input.trim_start_matches("\n").trim_end_matches("\n")
}
