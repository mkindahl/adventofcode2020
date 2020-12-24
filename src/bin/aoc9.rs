use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
    path::PathBuf,
};

fn is_invalid(n: usize, nums: &[usize]) -> bool {
    nums.iter()
        .enumerate()
        .flat_map(|(i, x)| repeat(x).zip(&nums[(i + 1)..]))
        .filter(|(&x, &y)| x + y == n)
        .all(|_| false)
}

fn part1(nums: &[usize], w: usize) -> Option<usize> {
    (w..nums.len())
        .find(|&i| is_invalid(nums[i], &nums[(i - w)..i]))
        .map(|i| nums[i])
}

fn part2(numbers: &[usize], number: usize) -> Option<usize> {
    for length in 2..numbers.len() {
        for pos in 0..numbers.len() - length {
            let range = &numbers[pos..pos + length];
            if range.iter().sum::<usize>() == number {
                return Some(range.iter().max().unwrap() + range.iter().min().unwrap());
            }
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc9.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let numbers: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();
    if let Some(number) = part1(&numbers, 25) {
        println!("Part 1: {}", number);
        println!("Part 2: {:?}", part2(&numbers, number));
    }
    Ok(())
}
