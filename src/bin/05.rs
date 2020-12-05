use std::io::{BufRead, Cursor};
use std::collections::HashSet;

static INPUT: &str = include_str!("./05.txt");

fn parse_bin(s: &str, one: char) -> u64 {
    s.chars().fold(0, |acc, c| acc * 2 + if c == one { 1 } else { 0 })
}

fn parse_seat(s: &str) -> (u64, u64) {
    let (col, row) = s.split_at(7);
    (parse_bin(col, 'B'), parse_bin(row, 'R'))
}

fn seat_id(s: &str) -> u64 {
    let (col, row) = parse_seat(s);
    col * 8 + row
}

fn main() {
    let seats = Cursor::new(INPUT).lines().into_iter().map(|line| seat_id(&line.unwrap())).collect::<HashSet<u64>>();
    for i in 1..=*seats.iter().max().unwrap() {
        if !seats.contains(&i) && seats.contains(&(i - 1)) && seats.contains(&(i + 1)) {
            println!("{}", i);
            break;
        }
    }
}
