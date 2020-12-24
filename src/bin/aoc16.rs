#![feature(str_split_once)]

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
    path::PathBuf,
    str::FromStr,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/aoc16.dat");
    let contents = read_to_string(path)?;
    let parts: Vec<_> = contents.split("\n\n").collect();
    let rules = parse_rules(parts[0]);
    let nearby: Vec<Vec<usize>> = parts[2]
        .split('\n')
        .skip(1)
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .collect();
    part1(&rules, &nearby);

    let your: Vec<Vec<usize>> = parts[1]
        .split('\n')
        .skip(1)
        .map(|line| {
            line.split(',')
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .collect();

    part2(your.first().unwrap(), &rules, &nearby);

    Ok(())
}

fn part1(rules: &HashMap<String, Rule>, tickets: &Vec<Vec<usize>>) {
    let mut found = Vec::new();
    'ticket: for ticket in tickets {
        let mut invalid: HashSet<usize> = ticket.iter().cloned().collect();
        for rule in rules {
            let found = get_invalid(rule.1, &ticket).iter().cloned().collect();
            invalid = invalid.intersection(&found).cloned().collect();
            if invalid.is_empty() {
                continue 'ticket;
            }
        }
        let mut ivec: Vec<usize> = invalid.iter().cloned().collect();
        found.append(&mut ivec);
    }
    println!("Part 1: {:?}", found.iter().sum::<usize>());
}

fn part2(your: &Vec<usize>, rules: &HashMap<String, Rule>, nearby: &Vec<Vec<usize>>) {
    let mut tickets: Vec<Vec<usize>> = nearby
        .iter()
        .filter(|ticket| is_valid_ticket(rules, ticket))
        .cloned()
        .collect();
    tickets.push(your.clone());
    let matching = compute_matching(&rules, &tickets);
    let mut order: Vec<usize> = (0..matching.len()).collect();
    order.sort_by(|x, y| matching[*x].len().cmp(&matching[*y].len()));
    println!("order: {:?}", order);
    if let Some(fields) = recurse(vec![], &order, rules, &tickets, &matching) {
        println!("fields: {:?}", fields);
        let answer: usize = order
            .iter()
            .zip(fields)
            .filter(|(_, n)| n.starts_with("departure"))
            .map(|(i, _)| your[*i])
            .product();

        println!("Part 2: {}", answer);
    }
}

fn compute_matching<'a>(
    rules: &'a HashMap<String, Rule>,
    tickets: &Vec<Vec<usize>>,
) -> Vec<Vec<&'a Rule>> {
    let mut matching = Vec::new();
    for index in 0..tickets[0].len() {
        let heads: Vec<usize> = tickets.iter().map(|ticket| ticket[index]).collect();
        let matches: Vec<_> = rules
            .iter()
            .filter_map(|(_, rule)| {
                if heads.iter().all(|&head| rule.good(head)) {
                    Some(rule)
                } else {
                    None
                }
            })
            .collect();
        matching.push(matches);
    }
    matching
}

fn recurse(
    path: Vec<String>,
    order: &[usize],
    rules: &HashMap<String, Rule>,
    tickets: &Vec<Vec<usize>>,
    matching: &Vec<Vec<&Rule>>,
) -> Option<Vec<String>> {
    if order.len() > 0 {
        //        println!("path: {:?}, index: {}", path, index);
        for rule in matching[order[0]]
            .iter()
            .filter(|rule| path.iter().all(|s| *s != rule.name))
        {
            let mut path = path.clone();
            path.push(rule.name.clone());
            if let Some(path) = recurse(path, &order[1..], rules, tickets, matching) {
                return Some(path);
            }
        }
        None
    } else {
        Some(path)
    }
}

fn is_valid_ticket(rules: &HashMap<String, Rule>, ticket: &Vec<usize>) -> bool {
    ticket
        .iter()
        .all(|val| rules.iter().any(|(_, rule)| rule.good(*val)))
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(line: &str) -> Result<Rule, ()> {
        let v: Vec<_> = line.split(':').collect();
        let name = v[0];
        let ranges = v[1]
            .split(" or ")
            .filter_map(|r| {
                r.split_once('-')
                    .map(|(x, y)| (x.trim().parse().unwrap(), y.trim().parse().unwrap()))
            })
            .collect();
        Ok(Rule {
            name: name.to_string(),
            ranges,
        })
    }
}

impl Rule {
    fn good(&self, n: usize) -> bool {
        self.ranges.iter().any(|(l, u)| *l <= n && n <= *u)
    }
}

fn get_invalid(rule: &Rule, ticket: &Vec<usize>) -> Vec<usize> {
    ticket.iter().filter(|&n| !rule.good(*n)).cloned().collect()
}

fn parse_rules(part: &str) -> HashMap<String, Rule> {
    part.split('\n')
        .map(|line| {
            let rule: Rule = line.parse().unwrap();
            (rule.name.clone(), rule)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_valid_ticket() {
        let rules = parse_rules("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50");
        assert!(is_valid_ticket(&rules, &vec![7, 3, 47]));
        assert!(!is_valid_ticket(&rules, &vec![40, 4, 50]));
        assert!(is_valid_ticket(&rules, &vec![55, 2, 20]));
        assert!(is_valid_ticket(&rules, &vec![38, 6, 12]));
    }
}
