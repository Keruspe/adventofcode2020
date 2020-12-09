static INPUT: &str = include_str!("./09.txt");

const WINDOW_SIZE: usize = 25;

fn pairs(input: &[usize]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, f)| {
            input.iter().skip(i + 1)
                .map(|s| (*f, *s))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = INPUT
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let first_wrong = *input
        .iter()
        .skip(WINDOW_SIZE)
        .enumerate()
        .find(|(i, u)| {
            pairs(&input[*i..][..WINDOW_SIZE])
                .iter()
                .all(|(f, s)| **u != f + s)
        })
        .unwrap()
        .1;
    println!("{}", first_wrong);
}
