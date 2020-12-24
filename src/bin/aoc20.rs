#![feature(str_split_once)]

use regex::Regex;
use std::{
    collections::HashMap, error::Error, fs::read_to_string, num::ParseIntError, path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc19.dat");
    let contents = read_to_string(path)?;
    let parts = contents.split_once("\n\n").unwrap();
    let tiles: Result<HashMap<usize, Body>, _> =
        parts.0.lines().map(|line| from_str(line)).collect();
    let rules = rules.unwrap();
    let messages: Vec<&str> = parts.1.lines().collect();
    println!("Part 1: {:?}", part1(&rules, &messages));
    println!("Part 2: {:?}", part2(&rules, &messages));
    Ok(())
}
