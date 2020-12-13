static INPUT: &str = include_str!("./13.txt");

fn main() {
    let timestamp = INPUT.lines().next().unwrap().parse::<usize>().unwrap();
    let bus_lines = INPUT.lines().skip(1).next().unwrap().split(',').filter_map(|bus| bus.parse::<usize>().ok()).collect::<Vec<_>>();
    let (next_bus, next_ts) = bus_lines.iter().map(|bus| (*bus, (0..).step_by(*bus).skip_while(|t| *t < timestamp).next().unwrap())).min_by_key(|(_, ts)| *ts).unwrap();
    println!("{}", next_bus * (next_ts - timestamp));
}
