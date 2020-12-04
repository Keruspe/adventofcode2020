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
    let scenarii = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let res: u64 = scenarii.iter().map(|(right, down)| {
        let mut trees = 0;
        for y in (*down..area.0.len()).step_by(*down) {
            if let Square::Tree = area.get((y / down) * right, y) {
                trees += 1;
            }
        }
        trees
    }).product();
    println!("{}", res);
}
