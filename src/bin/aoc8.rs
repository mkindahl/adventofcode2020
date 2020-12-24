use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Clone)]
enum Instr {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct Machine {
    pc: i32,
    acc: i32,
}

impl Instr {
    fn execute(&self, machine: &mut Machine) {
        use Instr::*;

        match self {
            Acc(val) => {
                machine.acc += val;
                machine.pc += 1;
            }
            Jmp(val) => {
                machine.pc += val;
            }
            Nop(_) => {
                machine.pc += 1;
            }
        }
    }
}

impl Machine {
    fn run(&mut self, program: &[Instr]) -> bool {
        let mut visited = HashSet::new();
        loop {
            if (self.pc + 1) as usize == program.len() {
                return true;
            }
            if visited.contains(&self.pc) {
                return false;
            }
            visited.insert(self.pc);
            let instr = &program[self.pc as usize];
            instr.execute(self);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc8.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let program = reader
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<Vec<Instr>, MachineError>>()
        .unwrap();

    // Part 1. Run the machine until it loops.
    let mut machine = Machine { pc: 0, acc: 0 };
    let terminated = machine.run(&program);
    println!("machine: {:?}, terminated: {}", machine, terminated);

    // Part 2. Try to change each instruction and run it to see if it
    // terminates properly.
    for i in 0..program.len() {
        let instr = match program[i] {
            Instr::Nop(v) => Instr::Jmp(v),
            Instr::Jmp(v) => Instr::Nop(v),
            _ => continue,
        };
        let mut program = program.clone();
        program[i] = instr;
        let mut machine = Machine { pc: 0, acc: 0 };
        if machine.run(&program) {
            println!("found: {:?}", machine);
            break;
        }
    }
    Ok(())
}

impl FromStr for Instr {
    type Err = MachineError;

    fn from_str(text: &str) -> Result<Instr, MachineError> {
        let words: Vec<&str> = text.split_whitespace().collect();
        match words[0] {
            "acc" => Ok(Instr::Acc(words[1].parse()?)),
            "jmp" => Ok(Instr::Jmp(words[1].parse()?)),
            "nop" => Ok(Instr::Nop(words[1].parse()?)),
            _ => Err(MachineError::BadInstr(text.to_string())),
        }
    }
}

#[derive(Debug)]
enum MachineError {
    BadInstr(String),
    IoError,
}

impl From<io::Error> for MachineError {
    fn from(_err: io::Error) -> MachineError {
        MachineError::IoError
    }
}

impl From<ParseIntError> for MachineError {
    fn from(err: ParseIntError) -> MachineError {
        MachineError::BadInstr(format!("bad instruction: {}", err))
    }
}
