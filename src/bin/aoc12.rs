use num::{complex::Complex, pow};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc12.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let actions: Result<Vec<Action>, AocError> = reader.lines().map(|line| line?.parse()).collect();
    let actions = actions.unwrap();
    let mut state = State::new(Complex::new(0, 0), Complex::new(1, 0));
    state.run(&actions, transform1);

    println!(
        "Part 1: {:?} = {}",
        state.pos,
        state.pos.re.abs() + state.pos.im.abs()
    );

    let mut state = State::new(Complex::new(0, 0), Complex::new(10, -1));
    state.run(&actions, transform2);
    println!(
        "Part 2: {:?} = {}",
        state.pos,
        state.pos.re.abs() + state.pos.im.abs()
    );

    Ok(())
}

const ROTATE_RIGHT: Complex<isize> = Complex::new(0, 1);
const ROTATE_LEFT: Complex<isize> = Complex::new(0, -1);
const MOVE_WEST: Complex<isize> = Complex::new(-1, 0);
const MOVE_EAST: Complex<isize> = Complex::new(1, 0);
const MOVE_SOUTH: Complex<isize> = Complex::new(0, 1);
const MOVE_NORTH: Complex<isize> = Complex::new(0, -1);

#[derive(Debug)]
struct State {
    pos: Complex<isize>,
    waypoint: Complex<isize>,
}

impl State {
    fn new(pos: Complex<isize>, waypoint: Complex<isize>) -> State {
        State { pos, waypoint }
    }

    fn run<F>(&mut self, actions: &[Action], transform: F)
    where
        F: Fn(&mut State, &Action),
    {
        actions.iter().for_each(|action| transform(self, action))
    }
}

fn transform1(state: &mut State, action: &Action) {
    match action {
        Action::North(n) => state.pos += n * MOVE_NORTH,
        Action::South(n) => state.pos += n * MOVE_SOUTH,
        Action::East(n) => state.pos += n * MOVE_EAST,
        Action::West(n) => state.pos += n * MOVE_WEST,
        Action::Right(d) => state.waypoint *= pow(ROTATE_RIGHT, (d / 90) as usize),
        Action::Left(d) => state.waypoint *= pow(ROTATE_LEFT, (d / 90) as usize),
        Action::Forward(n) => state.pos += n * state.waypoint,
    }
}

fn transform2(state: &mut State, action: &Action) {
    match action {
        Action::North(n) => state.waypoint += n * MOVE_NORTH,
        Action::South(n) => state.waypoint += n * MOVE_SOUTH,
        Action::East(n) => state.waypoint += n * MOVE_EAST,
        Action::West(n) => state.waypoint += n * MOVE_WEST,
        Action::Right(d) => state.waypoint *= pow(ROTATE_RIGHT, (d / 90) as usize),
        Action::Left(d) => state.waypoint *= pow(ROTATE_LEFT, (d / 90) as usize),
        Action::Forward(n) => state.pos += n * state.waypoint,
    }
}

#[derive(PartialEq, Debug)]
enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

#[derive(PartialEq, Debug)]
enum AocError {
    SyntaxError,
}

impl From<io::Error> for AocError {
    fn from(_err: io::Error) -> AocError {
        AocError::SyntaxError
    }
}
impl FromStr for Action {
    type Err = AocError;
    fn from_str(input: &str) -> Result<Action, AocError> {
        let n: isize = input
            .get(1..)
            .unwrap()
            .parse()
            .map_err(|_| AocError::SyntaxError)?;
        match input.get(0..1) {
            Some("N") => Ok(Action::North(n)),
            Some("S") => Ok(Action::South(n)),
            Some("E") => Ok(Action::East(n)),
            Some("W") => Ok(Action::West(n)),
            Some("F") => Ok(Action::Forward(n)),
            Some("L") => Ok(Action::Left(n)),
            Some("R") => Ok(Action::Right(n)),
            Some(_) | None => Err(AocError::SyntaxError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        assert_eq!("N10".parse::<Action>(), Ok(Action::North(10)));
        assert_eq!("S10".parse::<Action>(), Ok(Action::South(10)));
        assert_eq!("E10".parse::<Action>(), Ok(Action::East(10)));
        assert_eq!("W10".parse::<Action>(), Ok(Action::West(10)));
        assert_eq!("L10".parse::<Action>(), Ok(Action::Left(10)));
        assert_eq!("R10".parse::<Action>(), Ok(Action::Right(10)));
        assert_eq!("F10".parse::<Action>(), Ok(Action::Forward(10)));
    }
}
