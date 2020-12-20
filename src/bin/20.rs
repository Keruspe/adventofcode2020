static INPUT: &str = include_str!("./20.txt");

use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    left: String,
    left_rev: String,
    top: String,
    top_rev: String,
    right: String,
    right_rev: String,
    bottom: String,
    bottom_rev: String,
    data: Vec<Vec<u8>>,
}

fn serialize(data: &[u8], reverse: bool) -> String {
    data.iter().fold(String::new(), |acc, i| {
        if reverse {
            format!("{}{}", *i as char, acc)
        } else {
            format!("{}{}", acc, *i as char)
        }
    })
}

impl Tile {
    fn adjacent(&self, tiles: &[Tile]) -> u8 {
        let mut registry = HashSet::new();
        for tile in tiles {
            if tile.id == self.id {
                continue;
            }
            registry.insert(&tile.left);
            registry.insert(&tile.left_rev);
            registry.insert(&tile.top);
            registry.insert(&tile.top_rev);
            registry.insert(&tile.right);
            registry.insert(&tile.right_rev);
            registry.insert(&tile.bottom);
            registry.insert(&tile.bottom_rev);
        }
        let mut count = 0;
        if registry.contains(&self.left) {
            count += 1;
        }
        if registry.contains(&self.top) {
            count += 1;
        }
        if registry.contains(&self.right) {
            count += 1;
        }
        if registry.contains(&self.bottom) {
            count += 1;
        }
        count
    }
}

fn main() {
    let tiles = INPUT.split("\n\n").map(|tile| {
        let id = tile.lines().next().unwrap().trim_start_matches("Tile ").trim_end_matches(":").parse::<usize>().unwrap();
        let tile = tile.lines().skip(1).map(|line| line.as_bytes().to_vec()).collect::<Vec<_>>();
        let left = tile.iter().map(|l| l[0]).collect::<Vec<_>>();
        let top = tile[0].clone();
        let right = tile.iter().map(|l| *l.iter().last().unwrap()).collect::<Vec<_>>();
        let bottom = tile.iter().last().unwrap().clone();
        Tile {
            id,
            left: serialize(&left, false),
            left_rev: serialize(&left, true),
            top: serialize(&top, false),
            top_rev: serialize(&top, true),
            right: serialize(&right, false),
            right_rev: serialize(&right, true),
            bottom: serialize(&bottom, false),
            bottom_rev: serialize(&bottom, true),
            data: tile,
        }
    }).collect::<Vec<_>>();
    println!("{}", tiles.iter().filter(|tile| tile.adjacent(&tiles) == 2).map(|tile| tile.id).product::<usize>());
}
