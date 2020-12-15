use regex::Regex;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc14.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| Ok(line?))
        .collect::<Result<Vec<_>, io::Error>>()?;
    let mut mach = Mach::new(gen_masks_1);
    for line in &lines {
        mach.execute(line);
    }
    println!("Part 1: {}", mach.mem.values().sum::<usize>());

    let mut mach = Mach::new(gen_masks_2);

    for line in &lines {
        mach.execute(line);
    }
    println!("Part 2: {}", mach.mem.values().sum::<usize>());

    Ok(())
}

struct Mach<G>
where
    G: Fn(Vec<(usize, usize)>, char) -> Vec<(usize, usize)>,
{
    mem: HashMap<usize, usize>,
    masks: Vec<(usize, usize)>,
    mask_re: Regex,
    mem_re: Regex,
    gen: G,
}

impl<G> Mach<G>
where
    G: Fn(Vec<(usize, usize)>, char) -> Vec<(usize, usize)>,
{
    fn new(gen: G) -> Mach<G> {
        Mach {
            mem: HashMap::new(),
            masks: Vec::new(),
            mask_re: Regex::new(r"mask\s*=\s*([01X]+)").unwrap(),
            mem_re: Regex::new(r"mem\[(\d+)\]\s*=\s*(\d+)").unwrap(),
            gen,
        }
    }

    fn execute(&mut self, line: &str) {
        if let Some(caps) = self.mask_re.captures(line) {
            self.masks = caps
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .fold(vec![(0, !0)], &self.gen);
        } else if let Some(caps) = self.mem_re.captures(line) {
            let mem: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let val: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            for mask in &self.masks {
                let mem = (mem & mask.1) | mask.0;
                self.mem.insert(mem, val);
            }
        }
    }
}

fn gen_masks_1(masks: Vec<(usize, usize)>, c: char) -> Vec<(usize, usize)> {
    masks
        .iter()
        .map(|(x, y)| match c {
            '0' => (x << 1 | 0, y << 1 | 0),
            '1' => (x << 1 | 1, y << 1 | 1),
            'X' => (x << 1 | 0, y << 1 | 1),
            _ => todo!(),
        })
        .collect()
}

fn gen_masks_2(masks: Vec<(usize, usize)>, c: char) -> Vec<(usize, usize)> {
    match c {
        '0' => masks
            .iter()
            .map(|(x, y)| (x << 1 | 0, y << 1 | 1))
            .collect(),
        '1' => masks
            .iter()
            .map(|(x, y)| (x << 1 | 1, y << 1 | 1))
            .collect(),
        'X' => masks
            .iter()
            .flat_map(|(x, y)| vec![(x << 1 | 1, y << 1 | 0), (x << 1 | 0, y << 1 | 0)])
            .collect(),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let prog = [
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];
        let mut mach = Mach::new(gen_masks_1);
        for line in &prog {
            mach.execute(line);
        }
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            "000000000000000000000000000000X1001X"
                .chars()
                .fold(vec![(0, !0)], gen_masks_2)
                .iter()
                .map(|m| (42 & m.1) | m.0)
                .collect::<Vec<_>>(),
            vec![59, 58, 27, 26]
        );
    }
}
