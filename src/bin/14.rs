#![feature(str_split_once)]

static INPUT: &str = include_str!("./14.txt");

use std::str::FromStr;
use std::collections::BTreeMap;

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Assign(usize, u64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once(" = ").ok_or(())?;
        Ok(match lhs {
            "mask" => Instruction::Mask(rhs.parse()?),
            lhs if lhs.starts_with("mem[") && lhs.ends_with("]") => Instruction::Assign(lhs[4..(lhs.len() - 1)].parse().map_err(|_| ())?, rhs.parse().map_err(|_| ())?),
            _ => return Err(()),
        })
    }
}

#[derive(Clone, Debug)]
struct Mask {
    zeroes: u64,
    ones: u64,
    xs: Vec<usize>,
}

impl Mask {
    fn apply(&self, input: u64) -> u64 {
        input & self.zeroes | self.ones
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            zeroes: !0,
            ones: 0,
            xs: Vec::new(),
        }
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars().enumerate().fold(Ok(Mask::default()), |acc, (idx, c)| {
            acc.and_then(|mut acc| {
                match c {
                    '0' => acc.zeroes &= !(1 << (35 - idx)),
                    '1' => acc.ones |= 1 << (35 - idx),
                    'X' => acc.xs.push(35 - idx),
                    _ => return Err(()),
                }
                Ok(acc)
            })
        })
    }
}

fn main() {
    let mut memory: BTreeMap<usize, u64> = BTreeMap::new();
    let mut mask = Mask::default();
    let instructions = INPUT.lines().map(|line| line.parse::<Instruction>().unwrap()).collect::<Vec<_>>();
    for instr in &instructions {
        match instr {
            Instruction::Mask(m) => mask = m.clone(),
            Instruction::Assign(idx, val) => drop(memory.insert(*idx, mask.apply(*val))),
        }
    }
    println!("{}", memory.values().sum::<u64>());
}
