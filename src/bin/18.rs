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
        let mut rbegin = match s.rfind(" ") {
            Some(idx) => idx + 1,
            None => return Ok(Expr::Value(s.parse().map_err(|_| ())?)),
        };
        let mut lend = rbegin - 1;
        let mut rend = s.len();
        if s.chars().last().unwrap() == ')' {
            rend -= 1;
            let mut parens = 1;
            for (idx, c) in s.chars().collect::<Vec<_>>().iter().enumerate().rev().skip(1) {
                if *c == ')' {
                    parens += 1;
                } else if *c == '(' {
                    parens -= 1;
                }
                if parens == 0 {
                    rbegin = idx + 1;
                    if idx == 0 {
                        return s[rbegin..rend].parse();
                    } else {
                        lend = idx - 1;
                        break;
                    }
                }
            }
        }
        let rhs = &s[rbegin..rend];
        let (lhs, op) = &s[..lend].rsplit_once(" ").ok_or(())?;
        Ok(Expr::Operation {
            lhs: Box::new(lhs.parse()?),
            op: op.parse()?,
            rhs: Box::new(rhs.parse()?),
        })
    }
}

fn main() {
    let exprs: Vec<Expr> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    println!("{}", exprs.iter().map(Expr::evaluate).sum::<usize>());
}
