static INPUT: &str = include_str!("./12.txt");

use std::str::FromStr;
use std::ops::AddAssign;

enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, val) = s.split_at(1);
        let val = val.parse().map_err(|_| ())?;
        Ok(match a {
            "N" => Instruction::North(val),
            "S" => Instruction::South(val),
            "E" => Instruction::East(val),
            "W" => Instruction::West(val),
            "L" => Instruction::Left(val),
            "R" => Instruction::Right(val),
            "F" => Instruction::Forward(val),
            _ => return Err(()),
        })
    }
}

#[derive(Default, Clone, Copy)]
struct Position {
    east: usize,
    west: usize,
    north: usize,
    south: usize,
}

impl Position {
    fn move_north(&mut self, count: usize) {
        if self.south > count {
            self.south -= count
        } else {
            self.north += count - self.south;
            self.south = 0;
        }
    }

    fn move_south(&mut self, count: usize) {
        if self.north > count {
            self.north -= count
        } else {
            self.south += count - self.north;
            self.north = 0;
        }
    }

    fn move_east(&mut self, count: usize) {
        if self.west > count {
            self.west -= count
        } else {
            self.east += count - self.west;
            self.west = 0;
        }
    }

    fn move_west(&mut self, count: usize) {
        if self.east > count {
            self.east -= count
        } else {
            self.west += count - self.east;
            self.east = 0;
        }
    }

    fn manhattan(&self) -> usize {
        self.east + self.west + self.north + self.south
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        self.move_north(other.north);
        self.move_west(other.west);
        self.move_south(other.south);
        self.move_east(other.east);
    }
}

struct Waypoint {
    offset: Position,
}

impl Waypoint {
    fn rotate_left(&mut self, count: usize) {
        for _ in 0..count {
            let tmp = self.offset.north;
            self.offset.north = self.offset.east;
            self.offset.east = self.offset.south;
            self.offset.south = self.offset.west;
            self.offset.west = tmp;
        }
    }

    fn rotate_right(&mut self, count: usize) {
        self.rotate_left(4 - (count % 4))
    }
}

struct Boat {
    position: Position,
    waypoint: Waypoint,
}

impl Boat {
    fn follow_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::North(count) => self.waypoint.offset.move_north(*count),
            Instruction::East(count) => self.waypoint.offset.move_east(*count),
            Instruction::West(count) => self.waypoint.offset.move_west(*count),
            Instruction::South(count) => self.waypoint.offset.move_south(*count),
            Instruction::Left(count) => self.waypoint.rotate_left(count / 90),
            Instruction::Right(count) => self.waypoint.rotate_right(count / 90),
            Instruction::Forward(count) => for _ in 0..*count {
                self.position += self.waypoint.offset
            },
        }
    }
}

fn main() {
    let instructions = INPUT.lines().map(|line| line.parse::<Instruction>().unwrap()).collect::<Vec<_>>();
    let mut boat = Boat {
        position: Position::default(),
        waypoint: Waypoint {
            offset: Position {
                east: 10,
                north: 1,
                ..Position::default()
            }
        },
    };
    for instr in &instructions {
        boat.follow_instruction(instr);
    }
    println!("{}", boat.position.manhattan());
}
