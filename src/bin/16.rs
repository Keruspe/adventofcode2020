#![feature(str_split_once)]

static INPUT: &str = include_str!("./16.txt");

use std::str::FromStr;

struct Range(usize, usize);

impl Range {
    fn validate(&self, n: usize) -> bool {
        n >= self.0 && n <= self.1
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once("-").ok_or(())?;
        Ok(Self(min.parse().map_err(|_| ())?, max.parse().map_err(|_| ())?))
    }
}

struct Constraint {
    name: String,
    valid: Vec<Range>,
}

impl Constraint {
    fn validate(&self, n: usize) -> bool {
        self.valid.iter().any(|r| r.validate(n))
    }
}

impl FromStr for Constraint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, ranges) = s.split_once(": ").ok_or(())?;
        Ok(Self {
            name: name.to_string(),
            valid: ranges.split(" or ").map(|r| r.parse().map_err(|_| ())).collect::<Result<Vec<_>, Self::Err>>()?,
        })
    }
}

#[derive(Default)]
struct Constraints(Vec<Constraint>);

impl Constraints {
    fn validate(&self, n: usize) -> bool {
        self.0.iter().any(|r| r.validate(n))
    }
}

enum ParsingState {
    Constraints,
    Empty,
    SelfTicket,
    NearbyTickets,
}

#[derive(Default)]
struct Ticket(Vec<usize>);

impl FromStr for Ticket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.split(",").map(|r| r.parse().map_err(|_| ())).collect::<Result<Vec<_>, Self::Err>>()?))
    }
}

fn main() {
    let mut parsing_state = ParsingState::Constraints;
    let mut constraints = Constraints::default();
    let mut self_ticket = Ticket::default();
    let mut nearby_tickets: Vec<Ticket> = Vec::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            parsing_state = ParsingState::Empty;
        } else {
            match parsing_state {
                ParsingState::Constraints => constraints.0.push(line.parse().unwrap()),
                ParsingState::Empty => match line {
                    "your ticket:" => parsing_state = ParsingState::SelfTicket,
                    "nearby tickets:" => parsing_state = ParsingState::NearbyTickets,
                    _ => unreachable!(),
                }
                ParsingState::SelfTicket => self_ticket = line.parse().unwrap(),
                ParsingState::NearbyTickets => nearby_tickets.push(line.parse().unwrap()),
            }
        }
    }

    let part1 = nearby_tickets.iter().map(|t| t.0.iter().filter_map(|f| Some(f).filter(|f| !constraints.validate(**f))).sum::<usize>()).sum::<usize>();
    println!("{}", part1);
}
