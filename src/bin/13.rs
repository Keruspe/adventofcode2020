static INPUT: &str = include_str!("./13.txt");

fn main() {
    let timestamp = INPUT.lines().next().unwrap().parse::<usize>().unwrap();
    let bus_lines = INPUT.lines().skip(1).next().unwrap().split(',').enumerate().filter_map(|(idx, bus)| bus.parse::<usize>().ok().map(|bus| (idx, bus))).collect::<Vec<_>>();
    let (next_bus, next_ts) = bus_lines.iter().map(|(_, bus)| (*bus, (0..).step_by(*bus).skip_while(|t| *t < timestamp).next().unwrap())).min_by_key(|(_, ts)| *ts).unwrap();
    println!("{}", next_bus * (next_ts - timestamp));
    for timestamp in (0..).step_by(bus_lines[0].1) {
        let mut found: bool = true;
        for (delta, bus) in bus_lines.iter().skip(1) {
            if (timestamp + delta) % *bus != 0 {
                found = false;
                break;
            }
        }
        if found {
            println!("{}", timestamp);
            break;
        }
    }
}
