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
    println!("Part 1: {:?}", colors);

    let col = ("shiny".to_string(), "gold".to_string());
    println!("Part 2: {}", count_nodes(&map, &col));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

    #[test]
    fn test_rule_parse() {
        assert_eq!(
            parse("1 bright white bag"),
            (1, ("bright".to_string(), "white".to_string()))
        );
        assert_eq!(
            parse("2 muted yellow bags."),
            (2, ("muted".to_string(), "yellow".to_string()))
        );
    }

    #[test]
    fn test_count() {
        let examples = [
            (("faded".to_string(), "blue".to_string()), 0),
            (("dotted".to_string(), "black".to_string()), 0),
            (("vibrant".to_string(), "plum".to_string()), 11),
            (("dark".to_string(), "olive".to_string()), 7),
            (("shiny".to_string(), "gold".to_string()), 32),
        ];

        let mut map = HashMap::new();
        for line in INPUT.trim().lines() {
            let (key, tail) = line.trim().split_once("contain").unwrap();
            let bags: Vec<_> = tail.split(',').map(|b| parse(b)).collect();
            let rule = parse(key).1;
            assert!(map.insert(rule, bags).is_none());
        }

        for (col, expected) in &examples {
            assert_eq!(count_nodes(&map, col), *expected, "color: {:?}", col);
        }
    }
}
