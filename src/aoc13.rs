use std::{error::Error, fs::read_to_string, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc13.dat");
    let contents = read_to_string(path)?;
    let cvec: Vec<&str> = contents.split("\n").collect();
    let timestamp: isize = cvec[0].parse()?;
    let (line, departure) = cvec[1]
        .split(",")
        .filter_map(|s| s.parse::<isize>().ok())
        .map(|x| (x, x - timestamp % x))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    println!("Part 1: {}", line * departure);

    let solution = cvec[1]
        .split(",")
        .enumerate()
        .filter_map(|(e, s)| s.parse::<isize>().ok().map(|x| (e as isize, x)))
        .collect::<Vec<(isize, isize)>>();
    if let Some((mut a, m)) = solve(&solution) {
        if a < 0 {
            a = -a % m;
        }
        println!("Part 2: {}", a);
    }
    Ok(())
}

/// Chinese Reminder Theorem solver. Using iterated version of
/// extended euclidian (and 64 bit numbers, otherwise this would be a
/// pain).
fn solve(v: &[(isize, isize)]) -> Option<(isize, isize)> {
    let n: isize = v.iter().map(|e| e.1).product();
    let s: Option<isize> = v
        .iter()
        .map(|(a, m)| invert(n / m, *m).map(|i| a * (n / m) * i))
        .sum();
    match s {
        Some(v) => Some((v, n)),
        None => None,
    }
}

/// Extended Euclidian.
///
/// Compute x and y and g = gcd(a,b) such that ax + by = g. The values
/// x and y are called the *Bezout coefficients.*
fn euclidean(a: isize, b: isize) -> (isize, isize, isize) {
    let mut x = (a, b);
    let mut y = (1, 0);
    let mut z = (0, 1);
    while x.1 > 0 {
        let q = x.0 / x.1;
        let new_x = (x.1, x.0 - q * x.1);
        x = new_x;
        let new_y = (y.1, y.0 - q * y.1);
        y = new_y;
        let new_z = (z.1, z.0 - q * z.1);
        z = new_z;
    }
    (x.0, y.0, z.0)
}

fn invert(a: isize, m: isize) -> Option<isize> {
    let (g, x, _) = euclidean(a, m);
    if g == 1 {
        Some(x % m)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_euclidean() {
        assert_eq!(euclidean(240, 46), (2, -9, 47));
    }

    #[test]
    fn test_invert() {
        assert_eq!(invert(17, 43), Some(-5));
        assert_eq!(invert(240, 46), None);
    }

    #[test]
    fn test_solve() {
        let vs = [(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
        assert_eq!(solve(&vs), Some(-4231122, 3162341));
    }
}
