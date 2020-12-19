#![feature(str_split_once)]

static INPUT: &str = include_str!("./19.txt");

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Rule {
    Set(RuleSet),
    Alt(RuleSet, RuleSet),
    Value(char),
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('"') && s.ends_with('"') && s.len() == 3 {
            Self::Value(s.chars().nth(1).unwrap())
        } else {
            if let Some((left, right)) = s.split_once(" | ") {
                Self::Alt(left.parse()?, right.parse()?)
            } else {
                Self::Set(s.parse()?)
            }
        })
    }
}

impl Rule {
    fn validate<'a, 'b: 'a>(&self, s: &'a str, rules: &'b HashMap<usize, Rule>) -> HashSet<(bool, &'a str)> {
        match self {
            Self::Set(rs) => rs.validate(s, rules),
            Self::Alt(rs1, rs2) => {
                let mut res = rs1.validate(s, rules);
                res.extend(rs2.validate(s, rules));
                res
            }
            Self::Value(c) => {
                let mut res = HashSet::new();
                res.insert(if s.starts_with(*c) {
                    (true, &s[1..])
                } else {
                    (false, s)
                });
                res
            }
        }
    }
}

#[derive(Debug, Clone)]
struct RuleSet(Vec<usize>);

impl FromStr for RuleSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split_whitespace().map(|s| s.parse::<usize>().map_err(|_| ())).collect::<Result<_, _>>()?))
    }
}

impl RuleSet {
    fn validate<'a, 'b: 'a>(&self, s: &'a str, rules: &'b HashMap<usize, Rule>) -> HashSet<(bool, &'a str)> {
        let mut init = HashSet::new();
        init.insert((true, s));
        self.0.iter().fold(init, |acc, id| {
            acc.iter().flat_map(|(valid, s)| {
                if *valid {
                    rules.get(id).unwrap().validate(s, rules)
                } else {
                    let mut res = HashSet::new();
                    res.insert((*valid, *s));
                    res
                }
            }).collect()
        })
    }
}

fn main() {
    let (rules, messages) = INPUT.split_once("\n\n").unwrap();
    let mut rules = rules.lines().map(|line| {
        let (id, rule) = line.split_once(": ").unwrap();
        (id.parse::<usize>().unwrap(), rule.parse::<Rule>().unwrap())
    }).collect::<HashMap<_, _>>();
    let messages = messages.lines().collect::<Vec<_>>();
    let rule0 = rules.get(&0).unwrap().clone();
    let valid = messages.iter().filter(|m| rule0.validate(m, &rules).iter().any(|(valid, m)| *valid && m.is_empty())).count();
    println!("{}", valid);
    rules.insert(8, Rule::Alt(RuleSet(vec![42]), RuleSet(vec![42, 8])));
    rules.insert(11, Rule::Alt(RuleSet(vec![42, 31]), RuleSet(vec![42, 11, 31])));
    let valid = messages.iter().filter(|m| rule0.validate(m, &rules).iter().any(|(valid, m)| *valid && m.is_empty())).count();
    println!("{}", valid);
}
