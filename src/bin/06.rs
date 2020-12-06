use std::io::{BufRead, Cursor};

static INPUT: &str = include_str!("./06.txt");

const A: usize = 'a' as usize;

struct Group(u8, [u8; 26]);

impl Group {
    fn new() -> Self {
        Self(0, [0; 26])
    }

    fn add_person(&mut self, s: &str) {
        self.0 += 1;
        for c in s.as_bytes() {
            self.1[*c as usize - A] += 1;
        }
    }

    fn count(&self) -> usize {
        self.1.iter().filter(|c| **c == self.0).count()
    }
}

fn main() {
    let people = Cursor::new(INPUT).lines();
    let mut group = Group::new();
    let mut groups = Vec::new();
    for person in people {
        let person = person.unwrap();
        if person.is_empty() {
            groups.push(group);
            group = Group::new();
        } else {
            group.add_person(&person);
        }
    }
    let count: usize = groups.iter().map(Group::count).sum();
    println!("{}", count);
}
