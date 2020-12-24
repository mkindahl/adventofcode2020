#![feature(str_split_once)]

use std::{
    collections::{hash_map::RandomState, HashMap, HashSet},
    error::Error,
    fs::read_to_string,
    path::PathBuf,
};

fn read_passports(contents: &str) -> Vec<Vec<&str>> {
    contents
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<_>>())
        .collect()
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
    path.push("src/bin/aoc4.dat");

    let expected_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let expected: HashSet<_, RandomState> = expected_fields.iter().collect();
    let mut valid_passports = 0;
    let contents = read_to_string(path)?;
    for paragraph in read_passports(&contents) {
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
