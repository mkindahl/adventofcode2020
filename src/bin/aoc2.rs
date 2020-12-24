use regex::Regex;
use std::{
    error::Error,
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
struct Record {
    first: usize,
    second: usize,
    letter: char,
    passwd: String,
}

fn readpw(regex: &Regex, line: String) -> Record {
    let caps = regex.captures(&line).unwrap();
    Record {
        first: caps.get(1).unwrap().as_str().parse().unwrap(),
        second: caps.get(2).unwrap().as_str().parse().unwrap(),
        letter: caps.get(3).unwrap().as_str().chars().next().unwrap(),
        passwd: caps.get(4).unwrap().as_str().to_string(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc2.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let regex = Regex::new(r"(\d+)-(\d+) *(\w): *(.*)").unwrap();
    let records: Vec<_> = input
        .lines()
        .map(|line| readpw(&regex, line.unwrap()))
        .collect();

    let valid1: Vec<_> = records
        .iter()
        .filter(|record| {
            let count = record
                .passwd
                .chars()
                .filter(|&ch| ch == record.letter)
                .count();
            record.first <= count && count <= record.second
        })
        .collect();
    println!("Part 1: {:?}", valid1.len());

    let valid2: Vec<_> = records
        .iter()
        .filter(|record| {
            let first = record.passwd.chars().nth(record.first - 1).unwrap() == record.letter;
            let second = record.passwd.chars().nth(record.second - 1).unwrap() == record.letter;
            first && !second || !first && second
        })
        .collect();
    println!("Part 2: {:?}", valid2.len());

    Ok(())
}
