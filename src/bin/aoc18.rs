use std::{error::Error, fs::read_to_string, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc18.dat");
    let contents = read_to_string(path)?;
    let equations: Vec<&str> = contents.lines().collect();
    println!(
        "Part 1: {:?}",
        equations
            .iter()
            .map(|line| eval1(line))
            .sum::<Option<isize>>()
    );
    println!(
        "Part 1: {:?}",
        equations
            .iter()
            .map(|line| eval2(line))
            .sum::<Option<isize>>()
    );
    Ok(())
}

fn eval1(expr: &str) -> Option<isize> {
    let tokens: Vec<char> = expr.chars().filter(|c| !c.is_whitespace()).collect();
    let mut stack: Vec<isize> = vec![];
    let mut ops: Vec<char> = vec![];

    for token in tokens {
        match token {
            '0'..='9' => {
                stack.push(token.to_digit(10).unwrap() as isize);
                apply_ops(&mut stack, &mut ops, |c| c == '(');
            }
            '+' | '*' | '(' => ops.push(token),
            ')' => {
                ops.pop();
                apply_ops(&mut stack, &mut ops, |c| c == '(');
            }
            _ => todo!(),
        }
    }
    stack.pop()
}

fn eval2(expr: &str) -> Option<isize> {
    let tokens: Vec<char> = expr.chars().filter(|c| !c.is_whitespace()).collect();
    let mut stack: Vec<isize> = vec![];
    let mut ops: Vec<char> = vec![];

    for token in tokens {
        match token {
            '0'..='9' => {
                stack.push(token.to_digit(10).unwrap() as isize);
            }
            '+' => {
                apply_ops(&mut stack, &mut ops, |c| c == '(' || c == '*');
                ops.push(token);
            }
            '*' => {
                apply_ops(&mut stack, &mut ops, |c| c == '(');
                ops.push(token);
            }
            '(' => {
                ops.push(token);
            }
            ')' => {
                apply_ops(&mut stack, &mut ops, |c| c == '(');
                ops.pop();
            }
            _ => todo!(),
        }
    }
    apply_ops(&mut stack, &mut ops, |_| false);
    stack.pop()
}

fn apply_ops<F: Fn(char) -> bool>(stack: &mut Vec<isize>, ops: &mut Vec<char>, stop: F) {
    while let Some(op) = ops.pop() {
        if stop(op) {
            ops.push(op);
            break;
        }
        match op {
            '+' => apply(stack, |x, y| x + y),
            '*' => apply(stack, |x, y| x * y),
            _ => todo!(),
        }
    }
}

fn apply<F: Fn(isize, isize) -> isize>(stack: &mut Vec<isize>, f: F) {
    let lhs = stack.pop().unwrap();
    let rhs = stack.pop().unwrap();
    stack.push(f(lhs, rhs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval1() {
        assert_eq!(eval1("1 + 2 * 3 + 4 * 5 + 6"), Some(71));
        assert_eq!(eval1("1 + ( 2 * 3)"), Some(7));
        assert_eq!(eval1("2 * 3 + (4 * 5)"), Some(26));
        assert_eq!(eval1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(437));
        assert_eq!(
            eval1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Some(13632)
        );
    }
    #[test]
    fn test_eval2() {
        assert_eq!(eval2("1 + 2 * 3 + 4 * 5 + 6"), Some(231));
        assert_eq!(eval2("1 + ( 2 * 3)"), Some(7));
        assert_eq!(eval2("2 * 3 + (4 * 5)"), Some(46));
        assert_eq!(eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Some(1445));
        assert_eq!(
            eval2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Some(23340)
        );
    }
}
