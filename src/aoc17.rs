use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc17ex.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut board = Space::new();
    for (x, line) in reader.lines().enumerate() {
        let line = line?;
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                board.active.insert((x as isize, y as isize, 0));
            }
        }
    }

    for _ in 0..6 {
        board.evolve();
    }

    println!("active: {}", board.active.len());
    Ok(())
}

type Coord = (isize, isize, isize);

struct Space {
    active: HashSet<Coord>,
}

enum Change {
    Activate(isize, isize, isize),
    Deactivate(isize, isize, isize),
}

impl Space {
    fn new() -> Space {
        Space {
            active: HashSet::new(),
        }
    }

    fn print(&self) {
        for z in -3..7 {
            println!("z={}", z);
            for x in -3..7 {
                for y in -3..7 {
                    print!(
                        "{}",
                        if self.active.contains(&(x, y, z)) {
                            '#'
                        } else {
                            '.'
                        }
                    );
                }
                println!("");
            }
        }
    }

    fn evolve(&mut self) {
        let mut count: HashMap<Coord, isize> = HashMap::new();

        // Count active neighbours
        for (x, y, z) in self.active.iter() {
            //            println!("({},{},{}): {}", x, y, z, ch);
            *count.entry((x - 1, y - 1, z - 1)).or_insert(0) += 1;
            *count.entry((x - 1, y - 1, *z)).or_insert(0) += 1;
            *count.entry((x - 1, y - 1, z + 1)).or_insert(0) += 1;
            *count.entry((x - 1, *y, z - 1)).or_insert(0) += 1;
            *count.entry((x - 1, *y, *z)).or_insert(0) += 1;
            *count.entry((x - 1, *y, z + 1)).or_insert(0) += 1;
            *count.entry((x - 1, y + 1, z - 1)).or_insert(0) += 1;
            *count.entry((x - 1, y + 1, *z)).or_insert(0) += 1;
            *count.entry((x - 1, y + 1, z + 1)).or_insert(0) += 1;
            *count.entry((*x, y - 1, z - 1)).or_insert(0) += 1;
            *count.entry((*x, y - 1, *z)).or_insert(0) += 1;
            *count.entry((*x, y - 1, z + 1)).or_insert(0) += 1;
            *count.entry((*x, *y, z - 1)).or_insert(0) += 1;
            *count.entry((*x, *y, z + 1)).or_insert(0) += 1;
            *count.entry((*x, y + 1, z - 1)).or_insert(0) += 1;
            *count.entry((*x, y + 1, *z)).or_insert(0) += 1;
            *count.entry((*x, y + 1, z + 1)).or_insert(0) += 1;
            *count.entry((x + 1, y - 1, z - 1)).or_insert(0) += 1;
            *count.entry((x + 1, y - 1, *z)).or_insert(0) += 1;
            *count.entry((x + 1, y - 1, z + 1)).or_insert(0) += 1;
            *count.entry((x + 1, *y, z - 1)).or_insert(0) += 1;
            *count.entry((x + 1, *y, *z)).or_insert(0) += 1;
            *count.entry((x + 1, *y, z + 1)).or_insert(0) += 1;
            *count.entry((x + 1, y + 1, z - 1)).or_insert(0) += 1;
            *count.entry((x + 1, y + 1, *z)).or_insert(0) += 1;
            *count.entry((x + 1, y + 1, z + 1)).or_insert(0) += 1;
        }
        //        println!("count: {:?}", count);

        // Collect changes
        let mut changes = Vec::new();
        for ((x, y, z), cnt) in count.iter() {
            if self.active.contains(&(*x, *y, *z)) {
                if !(*cnt == 2 || *cnt == 3) {
                    changes.push(Change::Deactivate(*x, *y, *z));
                }
            } else if *cnt == 3 {
                changes.push(Change::Activate(*x, *y, *z));
            }
        }

        //    println!("changes: {:?}", changes);
        // Apply changes
        for change in changes {
            match change {
                Change::Activate(x, y, z) => {
                    self.active.insert((x, y, z));
                }
                Change::Deactivate(x, y, z) => {
                    self.active.remove(&(x, y, z));
                }
            }
        }
    }
}
