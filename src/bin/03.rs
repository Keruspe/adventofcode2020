use std::io::{BufRead, Cursor};

static INPUT: &str = include_str!("./03.txt");

#[derive(Copy, Clone)]
enum Square {
    Empty,
    Tree,
}

struct Line(Vec<Square>);

impl Line {
    fn get(&self, n: usize) -> Square {
        *self.0.get(n % self.0.len()).unwrap()
    }
}

struct Area(Vec<Line>);

impl Area {
    fn get(&self, x: usize, y: usize) -> Square {
        self.0.get(y).unwrap().get(x)
    }
}

fn main() {
    let area = Area(Cursor::new(INPUT).lines().into_iter().map(|line| {
        Line(line.unwrap().chars().map(|c| if c == '#' { Square::Tree } else { Square::Empty }).collect())
    }).collect());
    let mut trees = 0;
    for y in 1..area.0.len() {
        if let Square::Tree = area.get(y * 3, y) {
            trees += 1;
        }
    }
    println!("{}", trees);
}
