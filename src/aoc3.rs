use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn new(path: &Path) -> Result<Map, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let map = reader.split(0xA).collect::<Result<Vec<_>, _>>()?;
        Ok(Map { map })
    }

    fn trees(&self, right: usize, down: usize) -> u32 {
        let mut count = 0;
        let mut row = down;
        let mut col = right;
        while row < self.map.len() {
            if self.map[row][col] == 35 {
                count += 1;
            }
            col = (col + right) % self.map[row].len();
            row += down;
        }
        count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc3.dat");
    let map = Map::new(&path)?;

    // Part 1
    println!("trees: {}", map.trees(3, 1));

    // Part 2
    let product: u32 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| map.trees(*right, *down))
        .product();
    println!("product: {}", product);
    Ok(())
}
