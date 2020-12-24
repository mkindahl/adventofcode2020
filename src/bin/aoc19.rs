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
    let rules: Result<HashMap<usize, Body>, _> =
        parts.0.lines().map(|line| from_str(line)).collect();
    let rules = rules.unwrap();
    let messages: Vec<&str> = parts.1.lines().collect();
    println!("Part 1: {:?}", part1(&rules, &messages));
    println!("Part 2: {:?}", part2(&rules, &messages));
    Ok(())
}

fn part1(rules: &HashMap<usize, Body>, messages: &Vec<&str>) -> usize {
    let restr = format!("^{}$", build_regex(rules, 0));
    let regex = Regex::new(&restr).unwrap();
    messages.iter().filter(|msg| regex.is_match(msg)).count()
}

// For part 2, the rules 8 and 11 only exists at the top level in rule
// "0: 8 11", which means that the smallest match is "42 42 31", next
// is "42 42 42 31 31", etc. So we expand create a regexp that matches
// a series of at lest one 42s and a series of at least one 31s and
// ensure that there are more 42s than 31s (strictly).
fn part2(rules: &HashMap<usize, Body>, messages: &Vec<&str>) -> usize {
    let fortytwo = build_regex(rules, 42);
    let thirtyone = build_regex(rules, 31);
    let full = format!("^({}+)({}+)$", fortytwo, thirtyone);
    let fortytwo_re = Regex::new(&fortytwo).unwrap();
    let thirtyone_re = Regex::new(&thirtyone).unwrap();
    let full_re = Regex::new(&full).unwrap();
    messages
        .iter()
        .filter(|msg| {
            if let Some(cap) = full_re.captures(msg) {
                let first = fortytwo_re.replace_all(cap.get(1).unwrap().as_str(), "1");
                let second = thirtyone_re.replace_all(cap.get(2).unwrap().as_str(), "0");
                first.len() > second.len()
            } else {
                false
            }
        })
        .count()
}

#[derive(Debug)]
enum Body {
    Char(char),
    Alt(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct Rule {
    no: usize,
    body: Body,
}

#[derive(Debug)]
struct MyError;

fn from_str(line: &str) -> Result<(usize, Body), MyError> {
    if let Some((no, body)) = line.split_once(':') {
        let parts: Result<Vec<Vec<usize>>, ParseIntError> = body
            .split('|')
            .map(|p| p.split_whitespace().map(|s| s.parse()).collect())
            .collect();
        let body = match parts {
            Ok(parts) => Body::Alt(parts),
            Err(_) => Body::Char(body.trim().chars().skip(1).next().unwrap()),
        };
        Ok((no.parse().unwrap(), body))
    } else {
        Err(MyError)
    }
}

fn build_regex(rules: &HashMap<usize, Body>, start: usize) -> String {
    match rules.get(&start).unwrap() {
        Body::Alt(alts) => format!(
            "(?:{})",
            alts.iter()
                .map(|alt| alt
                    .iter()
                    .map(|&rule| build_regex(rules, rule))
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<_>>()
                .join("|")
        ),
        Body::Char(ch) => format!("{}", ch),
    }
}
