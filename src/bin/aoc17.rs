use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc17.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut active1 = HashSet::new();
    let mut active2 = HashSet::new();
    for (x, line) in reader.lines().enumerate() {
        let line = line?;
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                active1.insert((x as isize, y as isize, 0));
                active2.insert((x as isize, y as isize, 0, 0));
            }
        }
    }

    part1(active1);
    part2(active2);
    Ok(())
}

fn part1(mut active: HashSet<(isize, isize, isize)>) {
    for _ in 0..6 {
        evolve1(&mut active);
    }

    println!("active: {}", active.len());
}

fn part2(mut active: HashSet<(isize, isize, isize, isize)>) {
    for _ in 0..6 {
        evolve2(&mut active);
    }

    println!("active: {}", active.len());
}

fn evolve1(active: &mut HashSet<(isize, isize, isize)>) {
    let mut count = HashMap::new();

    // Count active neighbours
    for (x, y, z) in active.iter() {
        for i in x - 1..=x + 1 {
            for j in y - 1..=y + 1 {
                for k in z - 1..=z + 1 {
                    *count.entry((i, j, k)).or_insert(0) += 1;
                }
            }
        }
        count.entry((*x, *y, *z)).and_modify(|e| *e -= 1);
    }

    let next = count
        .iter()
        .filter_map(|((x, y, z), cnt)| {
            if active.contains(&(*x, *y, *z)) {
                if *cnt == 2 || *cnt == 3 {
                    Some((*x, *y, *z))
                } else {
                    None
                }
            } else if *cnt == 3 {
                Some((*x, *y, *z))
            } else {
                None
            }
        })
        .collect();

    *active = next;
}

fn evolve2(active: &mut HashSet<(isize, isize, isize, isize)>) {
    let mut count = HashMap::new();

    // Count active neighbours
    for (w, x, y, z) in active.iter() {
        for i in w - 1..=w + 1 {
            for j in x - 1..=x + 1 {
                for k in y - 1..=y + 1 {
                    for l in z - 1..=z + 1 {
                        *count.entry((i, j, k, l)).or_insert(0) += 1;
                    }
                }
            }
        }
        count.entry((*w, *x, *y, *z)).and_modify(|e| *e -= 1);
    }
    let next = count
        .iter()
        .filter_map(|((w, x, y, z), cnt)| {
            if active.contains(&(*w, *x, *y, *z)) {
                if *cnt == 2 || *cnt == 3 {
                    Some((*w, *x, *y, *z))
                } else {
                    None
                }
            } else if *cnt == 3 {
                Some((*w, *x, *y, *z))
            } else {
                None
            }
        })
        .collect();

    *active = next;
}
