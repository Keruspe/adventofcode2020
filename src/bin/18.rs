#![feature(str_split_once)]

static INPUT: &str = include_str!("./18.txt");

use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
enum Expr {
    Value(usize),
    Operation {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>,
    },
}

impl Expr {
    fn evaluate(&self) -> usize {
        match self {
            Expr::Value(val) => *val,
            Expr::Operation { lhs, op, rhs } => {
                let lhs = lhs.evaluate();
                let rhs = rhs.evaluate();
                match op {
                    Operator::Add => lhs + rhs,
                    Operator::Mul => lhs * rhs,
                }
            }
        }
    }
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.contains(" ") {
            return Ok(Expr::Value(s.parse().map_err(|_| ())?));
        }
        let mut toplevel_add = None;
        let mut toplevel_mul = None;
        let mut parens = 0;
        for (idx, c) in s.chars().enumerate() {
            match c {
                '(' => parens += 1,
                ')' => parens -= 1,
                '+' => {
                    if parens == 0 && toplevel_add.is_none() {
                        toplevel_add = Some(idx);
                    }
                }
                '*' => {
                    if parens == 0 {
                        toplevel_mul = Some(idx);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(idx) = toplevel_mul.or(toplevel_add) {
            Ok(Expr::Operation {
                lhs: Box::new(s[0..(idx - 1)].parse()?),
                op: s[idx..(idx + 1)].parse()?,
                rhs: Box::new(s[(idx + 2)..].parse()?),
            })
        } else {
            // means we're in toplevel parens, drop them
            s[1..(s.len() - 1)].parse()
        }
    }
}

fn main() {
    let exprs: Vec<Expr> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    println!("{}", exprs.iter().map(Expr::evaluate).sum::<usize>());
}
