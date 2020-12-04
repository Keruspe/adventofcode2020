use std::io::{BufRead, Cursor};
use std::collections::HashMap;

static INPUT: &str = include_str!("./04.txt");

const MANDATORY: &[(&str, fn(&str) -> bool)] = &[
    ("byr", |v| v.parse::<u64>().map_or(false, |year| year >= 1920 && year <= 2002)),
    ("iyr", |v| v.parse::<u64>().map_or(false, |year| year >= 2010 && year <= 2020)),
    ("eyr", |v| v.parse::<u64>().map_or(false, |year| year >= 2020 && year <= 2030)),
    ("hgt", |v| {
        let num = v.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();
        let ext = v.chars().skip_while(|c| c.is_digit(10)).collect::<String>();
        match ext.as_str() {
            "cm" => num >= 150 && num <= 193,
            "in" => num >= 59 && num <= 76,
            _ => false
        }
    }),
    ("hcl", |v| v.len() == 7 && v.chars().next().unwrap() == '#' && v.chars().skip(1).all(|c| c.is_digit(10) || (c >= 'a' && c <= 'f'))),
    ("ecl", |v| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v)),
    ("pid", |v| v.len() == 9 && v.chars().all(|c| c.is_digit(10))),
];

fn main() {
    let lines = Cursor::new(INPUT).lines();
    let mut valid = 0;
    let mut current : HashMap<String, String> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            if MANDATORY.iter().filter(|(m, cond)| current.get(*m).map_or(true, |v| !cond(v))).count() == 0 {
                valid += 1;
            }
            current = HashMap::new();
            continue;
        }
        for part in line.split(' ') {
            let mut part = part.split(':');
            current.insert(part.next().unwrap().to_string(), part.next().unwrap().to_string());
        }
    }
    println!("{}", valid);
}
