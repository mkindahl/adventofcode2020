#![feature(iterator_fold_self)]

use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

fn read_forms(path: &Path) -> Result<Vec<Vec<String>>, io::Error> {
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

fn part1(forms: &Vec<Vec<String>>) -> usize {
    forms
        .iter()
        .map(|group| {
            let result: HashSet<_, RandomState> = group
                .iter()
                .map(|s| s.chars().collect::<HashSet<_, RandomState>>())
                .fold_first(|a, s| a.union(&s).map(|s| *s).collect())
                .unwrap();
            result.len()
        })
        .sum::<usize>()
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc6.dat");
    let forms = read_forms(&path).unwrap();
    // Part 1
    println!("result: {:?}", part1(&forms));

    // Part 2
    let result: usize = forms
        .iter()
        .map(|group| {
            let result: HashSet<_, RandomState> = group
                .iter()
                .map(|s| HashSet::from_iter(s.chars()))
                .fold_first(|a, s| a.intersection(&s).map(|s| *s).collect())
                .unwrap();
            result.len()
        })
        .sum();
    println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(&vec![vec!["abc".to_string()]]), 3);
    }
}
