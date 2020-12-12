static INPUT: &str = include_str!("./12.txt");

use std::str::FromStr;

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

#[derive(Clone, Copy)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn next(&self, instr: &Instruction) -> Self {
        match instr {
            Instruction::Right(angle) => self.turn_right((angle / 90) %  4),
            Instruction::Left(angle) => self.turn_left((angle / 90) %  4),
            _ => *self
        }
    }

    fn turn_right(&self, count: usize) -> Self {
        if count == 0 {
            *self
        } else {
            match self {
                Direction::East => Direction::South.turn_right(count - 1),
                Direction::South => Direction::West.turn_right(count - 1),
                Direction::West => Direction::North.turn_right(count - 1),
                Direction::North => Direction::East.turn_right(count - 1),
            }
        }
    }

    fn turn_left(&self, count: usize) -> Self {
        if count == 0 {
            *self
        } else {
            match self {
                Direction::East => Direction::North.turn_left(count - 1),
                Direction::North => Direction::West.turn_left(count - 1),
                Direction::West => Direction::South.turn_left(count - 1),
                Direction::South => Direction::East.turn_left(count - 1),
            }
        }
    }
}

#[derive(Default)]
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

struct Boat {
    position: Position,
    direction: Direction,
}

impl Boat {
    fn follow_instruction(&mut self, instr: &Instruction) {
        self.direction = self.direction.next(instr);
        match instr {
            Instruction::North(count) => self.position.move_north(*count),
            Instruction::East(count) => self.position.move_east(*count),
            Instruction::West(count) => self.position.move_west(*count),
            Instruction::South(count) => self.position.move_south(*count),
            Instruction::Forward(count) => match self.direction {
                Direction::North => self.position.move_north(*count),
                Direction::East => self.position.move_east(*count),
                Direction::West => self.position.move_west(*count),
                Direction::South => self.position.move_south(*count),
            }
            _ => {}
        }
    }
}

fn main() {
    let instructions = INPUT.lines().map(|line| line.parse::<Instruction>().unwrap()).collect::<Vec<_>>();
    let mut boat = Boat {
        position: Position::default(),
        direction: Direction::East,
    };
    for instr in &instructions {
        boat.follow_instruction(instr);
    }
    println!("{}", boat.position.manhattan());
}
