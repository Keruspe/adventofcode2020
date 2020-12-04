use std::io::{BufRead, Cursor};

static INPUT: &str = include_str!("./02.txt");

fn main() {
    let entries = Cursor::new(INPUT).lines().into_iter().map(|line| {
        let line = line.unwrap();
        let mut line = line.split(' ');
        let range = line.next().unwrap();
        let letter = line.next().unwrap().chars().next().unwrap();
        let password = line.next().unwrap();
        let mut range = range.split('-');
        let min = range.next().unwrap().parse::<usize>().unwrap();
        let max = range.next().unwrap().parse::<usize>().unwrap();
        (min, max, letter, password.to_string())
    }).collect::<Vec<_>>();

    let count = entries.iter().filter(|(min, max, letter, password)| {
        let count = password.chars().filter(|c| c == letter).count();
        count >= *min && count <= *max
    }).count();

    println!("{}", count);
}
