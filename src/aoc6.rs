#![feature(iterator_fold_self)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;

fn read_forms(contents: &str) -> Vec<Vec<&str>> {
    contents
        .split("\n\n")
        .map(|group| group.lines().collect::<Vec<_>>())
        .collect()
}

fn part1(forms: &[Vec<&str>]) -> usize {
    forms
        .iter()
        .map(|group| {
            let result: HashSet<_> = group
                .iter()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .fold_first(|a, s| a.union(&s).cloned().collect())
                .unwrap();
            result.len()
        })
        .sum::<usize>()
}

fn part2(forms: &[Vec<&str>]) -> usize {
    forms
        .iter()
        .map(|group| {
            let result: HashSet<_> = group
                .iter()
                .map(|s| s.chars().collect::<HashSet<_>>())
                .fold_first(|a, s| a.intersection(&s).cloned().collect())
                .unwrap();
            result.len()
        })
        .sum::<usize>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc6.dat");
    let contents = read_to_string(path)?;
    let forms = read_forms(&contents);

    // Part 1
    println!("part 1: {:?}", part1(&forms));

    // Part 2
    println!("part 2: {:?}", part2(&forms));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&vec![vec!["abc"]]), 3);
        assert_eq!(part1(&vec![vec!["a", "b", "c"]]), 3);
        assert_eq!(part1(&vec![vec!["ab", "ac"]]), 3);
        assert_eq!(part1(&vec![vec!["a", "a", "a", "a"]]), 1);
        assert_eq!(part1(&vec![vec!["b"]]), 1);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&vec![vec!["abc"]]), 3);
        assert_eq!(part2(&vec![vec!["a", "b", "c"]]), 0);
        assert_eq!(part2(&vec![vec!["ab", "ac"]]), 1);
        assert_eq!(part2(&vec![vec!["a", "a", "a", "a"]]), 1);
        assert_eq!(part2(&vec![vec!["b"]]), 1);
    }
}
