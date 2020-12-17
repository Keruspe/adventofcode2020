static INPUT: &str = include_str!("./17.txt");

use std::collections::HashSet;

const INIT_LOOPS: usize = 6;

fn main() {
    let mut active = HashSet::new();
    let mut min_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut min_z: i64 = 0;
    let mut min_w: i64 = 0;
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    let mut max_z: i64 = 0;
    let mut max_w: i64 = 0;
    for (x, line) in INPUT.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                active.insert((x as i64, y as i64, 0i64, 0i64));
            }
            max_y = y as i64;
        }
        max_x = x as i64;
    }
    for _ in 0..INIT_LOOPS {
        min_x -= 1;
        min_y -= 1;
        min_z -= 1;
        min_w -= 1;
        max_x += 1;
        max_y += 1;
        max_z += 1;
        max_w += 1;
        let current = active.clone();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    for w in min_w..=max_w {
                        let mut active_neighbours = 0;
                        for xx in (x - 1)..=(x + 1) {
                            for yy in (y - 1)..=(y + 1) {
                                for zz in (z - 1)..=(z + 1) {
                                    for ww in (w - 1)..=(w + 1) {
                                        if xx != x || yy != y || zz != z || ww != w {
                                            if current.contains(&(xx, yy, zz, ww)) {
                                                active_neighbours += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if current.contains(&(x, y, z, w)) {
                            if active_neighbours != 2 && active_neighbours != 3 {
                                active.remove(&(x, y, z, w));
                            }
                        } else {
                            if active_neighbours == 3 {
                                active.insert((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{}", active.len());
}
