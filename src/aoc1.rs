use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::repeat;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc1.dat");
    let file = File::open(path)?;
    let input = BufReader::new(file);
    let nums: Vec<u32> = input
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Part 1
    let candidates: Vec<_> = nums
        .iter()
        .enumerate()
        .flat_map(|(i, x)| repeat(x).zip(&nums[(i + 1)..]))
        .filter(|(&x, &y)| x + y == 2020)
        .collect();

    for (x, y) in candidates {
        println!("{} * {} = {}", x, y, x * y);
    }

    // Part 2
    let candidates: Vec<_> = nums
        .iter()
        .enumerate()
        .flat_map(|(i, x)| repeat(x).zip(&nums[(i + 1)..]).enumerate())
        .flat_map(|(j, (x, y))| repeat((x, y)).zip(&nums[(j + 1)..]))
        .filter(|((&x, &y), &z)| x + y + z == 2020)
        .collect();

    for ((x, y), z) in candidates {
        println!("{} * {} * {} = {}", x, y, z, x * y * z);
    }

    Ok(())
}
