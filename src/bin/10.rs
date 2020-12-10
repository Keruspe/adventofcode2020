static INPUT: &str = include_str!("./10.txt");

fn main() {
    let mut joltages = INPUT.lines().map(|line| line.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let device_joltage = joltages.iter().max().unwrap() + 3;
    joltages.push(0);
    joltages.push(device_joltage);
    joltages.sort();
    println!("device built-in joltage: {}", device_joltage);
    let mut diff_one = 0;
    let mut diff_two = 0;
    let mut diff_three = 0;
    for i in 1..joltages.len() {
        match joltages[i] - joltages[i - 1] {
            1 => diff_one += 1,
            2 => diff_two += 1,
            3 => diff_three += 1,
            _ => unreachable!(),
        }
    }
    println!("Diff is 1: {}", diff_one);
    println!("Diff is 2: {}", diff_two);
    println!("Diff is 3: {}", diff_three);
    println!("Part 1: {}", diff_one * diff_three);
}
