use std::collections::HashMap;

fn main() {
    let numbers = [6, 3, 15, 13, 1, 0];
    println!("Part 1: {:?}", play(&numbers, 2020));
    println!("Part 2: {:?}", play(&numbers, 30000000));
}

fn play(numbers: &[usize], nth: usize) -> usize {
    let mut record: HashMap<_, _> = numbers.iter().enumerate().map(|(x, &y)| (y, x)).collect();
    let mut spoken = numbers[numbers.len() - 1];
    for turn in (numbers.len() - 1)..(nth - 1) {
        if turn % 100000 == 0 {
            println!("{}", turn);
        }
        spoken = match record.insert(spoken, turn) {
            None => 0,
            Some(last) => turn - last,
        };
    }
    spoken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(play(&[0, 3, 6], 2020), 436);
        assert_eq!(play(&[1, 3, 2], 2020), 1);
        assert_eq!(play(&[2, 1, 3], 2020), 10);
    }
}
