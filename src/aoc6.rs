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
        if line.len() == 0 {
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

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc6.dat");
    let forms = read_forms(&path).unwrap();
    // Part 1
    let result: usize = forms
        .iter()
        .map(|group| {
            let result: HashSet<_, RandomState> = group
                .iter()
                .map(|s| HashSet::from_iter(s.chars()))
                .fold_first(|a, s| a.union(&s).map(|s| *s).collect())
                .unwrap();
            result.len()
        })
        .sum();

    println!("result: {:?}", result);

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
    #[test]
    fn test() {}
}
