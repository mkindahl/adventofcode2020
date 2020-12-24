use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc10.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let jolts: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();
    println!("Part 1: {:?}", part1(jolts.clone()));
    println!("Part 2: {:?}", part2(jolts));
    Ok(())
}

fn part1(mut jolts: Vec<usize>) -> usize {
    jolts.sort_unstable();
    let mut prev = 0;
    let mut dist = [0, 0, 0, 1];
    for j in jolts {
        dist[j - prev] += 1;
        prev = j;
    }
    dist[1] * dist[3]
}

fn part2(jolts: Vec<usize>) -> usize {
    Memo::new(jolts).arrs(0, 0)
}

/// Memoizer for the sub-solutions.
struct Memo {
    adapters: Vec<usize>,
    done: Vec<Option<usize>>,
}

impl Memo {
    fn new(mut adapters: Vec<usize>) -> Memo {
        let mut done = vec![];
        adapters.sort_unstable();
        adapters.push(adapters.last().unwrap() + 3);
        done.resize(adapters.len(), None);
        Memo { adapters, done }
    }

    fn remember(&mut self, idx: usize, val: usize) -> usize {
        self.done[idx] = Some(val);
        val
    }

    fn arrs(&mut self, curr: usize, idx: usize) -> usize {
        if idx < self.adapters.len() - 1 {
            if let Some(value) = self.done[idx] {
                return value;
            }

            let candidates: Vec<_> = (idx..self.adapters.len())
                .take_while(|&i| self.adapters[i] - curr <= 3)
                .collect();

            let result = candidates
                .into_iter()
                .map(|i| self.arrs(self.adapters[i], i + 1))
                .sum();
            self.remember(idx, result)
        } else {
            self.remember(idx, 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc10_part2() {
        let jolts = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part2(jolts), 8);
        let jolts = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part2(jolts), 19208);
    }
}
