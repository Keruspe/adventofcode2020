static INPUT: &str = include_str!("./15.txt");

use std::collections::HashMap;

fn run(init: &[usize], boundary: usize) {
    let mut previous = None;
    let mut current = 0;
    let mut state = HashMap::new();
    for (turn, n) in init.iter().enumerate() {
        current = *n;
        previous = state.get(&current).copied();
        state.insert(current, turn + 1);
    }
    for turn in init.len()..boundary {
        if let Some(t) = previous {
            current = turn - t;
        } else {
            current = 0;
        }
        previous = state.get(&current).copied();
        state.insert(current, turn + 1);
    }
    println!("{}", current);
}

fn main() {
    let init = INPUT.lines().next().unwrap().split(",").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    run(&init[..], 2020);
    run(&init[..], 30000000);
}
