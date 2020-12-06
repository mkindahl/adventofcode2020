use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

fn seat_id<S: Into<String>>(line: S) -> u32 {
    // Read the bits as a binary number and split it into the parts
    // afterwards.
    let mut result: u32 = 0;
    for ch in line.into().chars() {
        result = (result << 1) | (ch == 'R' || ch == 'B') as u32;
    }
    8 * (result >> 3) + (result & 0x7)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc5.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut seats = reader
        .lines()
        .map(|line| line.map(seat_id))
        .collect::<Result<Vec<_>, _>>()?;
    seats.sort_unstable();

    // Part 1. Get the maximum seat number of all.
    let max = *seats.last().unwrap();
    println!("max seat: {}", max);

    // Part 2. Since we know that the missing seat -1 and +1 are in
    // the list, we can just check the range of seats and can ignore
    // row number. We do that by zipping with the complete range of
    // seats and pick the first element where the seat numbers do not
    // match.
    let min = *seats.first().unwrap();
    let missing = (min..max).zip(seats).find(|(x, y)| *x != *y).unwrap();
    println!("missing seat: {}", missing.0);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
