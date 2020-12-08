#![feature(str_split_once)]

use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

type Bag = (String, String);

fn parse(text: &str) -> (usize, Bag) {
    let words: Vec<&str> = text
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect();
    assert!(words.len() >= 3 && words.len() <= 4);
    match words[0].parse() {
        Ok(count) => (count, (words[1].to_string(), words[2].to_string())),
        Err(_) => (0, (words[0].to_string(), words[1].to_string())),
    }
}

fn can_contain(map: &HashMap<Bag, Vec<(usize, Bag)>>, start: &Bag, target: &Bag) -> bool {
    match map.get(&start) {
        Some(bags) => bags
            .iter()
            .any(|(_, bag)| bag == target || can_contain(map, bag, target)),
        None => false,
    }
}

fn count_nodes(map: &HashMap<Bag, Vec<(usize, Bag)>>, start: &Bag) -> usize {
    match map.get(&start) {
        Some(bags) => bags
            .iter()
            .map(|(cnt, bag)| cnt * (count_nodes(map, bag) + 1))
            .sum(),
        None => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc7.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let (key, tail) = line.split_once("contain").unwrap();
        let bags: Vec<_> = tail.split(',').map(|b| parse(b)).collect();
        let rule = parse(key).1;
        assert!(map.insert(rule, bags).is_none());
    }
    let target = ("shiny".to_string(), "gold".to_string());
    let colors = map
        .keys()
        .filter(|bag| can_contain(&map, bag, &target))
        .count();
    println!("answer: {:?}", colors);

    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_rule_parse() {
        assert_eq!(
            parse("1 bright white bag"),
            (1, ("bright".to_string(), "white".to_string()))
        );
        assert_eq!(
            parse("2 muted yellow bags."),
            (1, ("muted".to_string(), "yellow".to_string()))
        );
    }

    #[test]
    fn test_count() {
        for col in [
            ("faded".to_string(), "blue".to_string()),
            ("dotted".to_string(), "black".to_string()),
            ("vibrant".to_string(), "plum".to_string()),
            ("dark".to_string(), "olive".to_string()),
            ("shiny".to_string(), "gold".to_string()),
        ]
        .iter()
        {
            println!("{:?}: {}", col, count_nodes(&map, &col));
        }
    }
}
