#![feature(str_split_once)]

use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc19.dat");
    let contents = read_to_string(path)?;
    let regex = Regex::new(r"Tile (\d+):").unwrap();
    let tiles: Result<Vec<Tile>, Error> = contents
        .split("\n\n")
        .map(|para| parse_tile(&regex, para.lines().collect()))
        .collect();

    println!("Part 1: {:?}", part1());
    println!("Part 2: {:?}", part2());
    Ok(())
}

struct Tile {
    number: usize,
    data: HashMap<(usize, usize), char>,
}

fn parse_tile(regex: &Regex, lines: Vec<&str>) -> Result<Tile, Error> {
    if let Some(cap) = regex.captures(lines[0]) {
        let number = cap.get(1).unwrap().as_str().parse().unwrap();
        let data: HashMap<(usize, usize), char> = lines[1..]
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| ((row, col), ch))
                    .collect::<Vec<((usize, usize), char)>>()
            })
            .collect();
        Ok(Tile { number, data })
    } else {
        Err(Error)
    }
}

fn part1() {}

fn part2() {}

struct Error;
