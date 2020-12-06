#![feature(str_split_once)]

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn read_passports(path: &Path) -> Result<Vec<Vec<String>>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut paragraphs = Vec::new();
    let mut paragraph = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            paragraphs.push(paragraph.clone());
            paragraph.clear();
        } else {
            paragraph.push(line);
        }
    }

    paragraphs.push(paragraph.clone());
    paragraph.clear();
    Ok(paragraphs)
}

fn check_field(key: &str, value: &str) -> bool {
    match key {
        "byr" => {
            let year: u32 = value.parse().unwrap();
            value.len() == 4 && year >= 1920 && year <= 2002
        }
        "iyr" => {
            let year: u32 = value.parse().unwrap();
            value.len() == 4 && year >= 2010 && year <= 2020
        }
        "eyr" => {
            let year: u32 = value.parse().unwrap();
            value.len() == 4 && year >= 2020 && year <= 2030
        }
        "hgt" => {
            let (val, sfx) = value.split_at(value.len() - 2);
            match (val.parse::<u32>(), sfx) {
                (Ok(num), "cm") => (150..=193).contains(&num),
                (Ok(num), "in") => (59..=76).contains(&num),
                (Err(_), _) | (Ok(_), _) => false,
            }
        }
        "hcl" => {
            value.len() == 7
                && value.chars().take(1).next() == Some('#')
                && value.chars().skip(1).all(|ch| ch.is_digit(16))
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .any(|&cand| cand == value),
        "pid" => value.len() == 9 && value.chars().all(|ch| ch.is_digit(10)),
        _ => true, // Just ignore the fields
    }
}

fn valid_fields<S>(passport: &HashMap<&str, &str, S>) -> bool {
    passport
        .iter()
        .all(|(&key, &value)| check_field(key, value))
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc4.dat");

    let expected_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let expected: HashSet<_, RandomState> = expected_fields.iter().collect();
    let mut valid_passports = 0;
    for paragraph in read_passports(&path)? {
        let passport: HashMap<&str, &str, RandomState> = paragraph
            .iter()
            .flat_map(|line| line.split(' '))
            .filter_map(|field| field.split_once(':'))
            .collect();
        let keys: HashSet<_, RandomState> = passport.keys().collect();
        if expected.is_subset(&keys) && valid_fields(&passport) {
            valid_passports += 1;
        }
    }
    println!("valid passports: {}", valid_passports);
    Ok(())
}
