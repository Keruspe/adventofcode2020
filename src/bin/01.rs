use std::io::{BufRead, Cursor};

static INPUT: &str = include_str!("./01.txt");

fn main() {
    let numbers = Cursor::new(INPUT).lines().into_iter().map(|line| line.unwrap().parse().unwrap()).collect::<Vec<u64>>();
    for x in &numbers {
        for y in &numbers {
            for z in &numbers {
                if x + y + z == 2020 {
                    println!("{}", x * y * z);
                    return;
                }
            }
        }
    }
}
