#![feature(str_split_once)]

use std::collections::HashMap;

static INPUT: &str = include_str!("./07.txt");

const TARGET_COLOUR: &str = "shiny gold";

fn can_contain(bags: &HashMap<String, Vec<(u8, String)>>, current_colour: &str, target_colour: &str) -> bool {
    bags.get(current_colour)
        .unwrap()
        .iter()
        .any(|(_, colour)| {
            colour == target_colour || can_contain(bags, colour, target_colour)
        })
}

fn must_contain(bags: &HashMap<String, Vec<(u8, String)>>, colour: &str) -> u64 {
    bags.get(colour)
        .unwrap()
        .iter()
        .map(|(number, colour)| {
            (*number as u64) * (1 + must_contain(bags, colour))
        })
        .sum()
}

fn main() {
    let bags = INPUT.lines().map(|line| {
        let (container, contents) = line.split_once(" bags contain ").unwrap();
        let contents = if contents == "no other bags." {
            Vec::new()
        } else {
            contents.trim_end_matches('.')
                .split(", ")
                .map(|bag| bag.trim_end_matches("s"))
                .map(|bag| bag.trim_end_matches(" bag"))
                .map(|bag| bag.split_once(' ').unwrap())
                .map(|(number, colour)| (number.parse::<u8>().unwrap(), colour.to_string()))
                .collect::<Vec<_>>()
        };
        (container.to_string(), contents)
    }).collect::<HashMap<String, Vec<(u8, String)>>>();

    let count = bags.keys().filter(|colour| can_contain(&bags, colour, TARGET_COLOUR)).count();
    println!("{}", count);

    println!("{}", must_contain(&bags, TARGET_COLOUR));
}
