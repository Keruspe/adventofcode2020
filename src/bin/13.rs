static INPUT: &str = include_str!("./13.txt");

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn main() {
    let timestamp = INPUT.lines().next().unwrap().parse::<i64>().unwrap();
    let bus_lines = INPUT.lines().skip(1).next().unwrap().split(',').enumerate().filter_map(|(idx, bus)| bus.parse::<i64>().ok().map(|bus| (idx as i64, bus))).collect::<Vec<_>>();
    let (next_bus, next_ts) = bus_lines.iter().map(|(_, bus)| (*bus, (0..).step_by(*bus as usize).skip_while(|t| *t < timestamp).next().unwrap())).min_by_key(|(_, ts)| *ts).unwrap();
    println!("{}", next_bus * (next_ts - timestamp));
    let product = bus_lines.iter().map(|(_, bus)| *bus).product::<i64>();
    let ts = bus_lines.iter().fold(0, |acc, (delta, bus)| acc + (bus - delta) * mod_inv(product / bus, *bus).unwrap() * (product / bus)) % product;
    println!("{}", ts);
}
