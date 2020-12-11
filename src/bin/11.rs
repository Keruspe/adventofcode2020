static INPUT: &str = include_str!("./11.txt");

const TOLERANCE: u8 = 5;
const DIRECT_ONLY: bool = false;

#[derive(PartialEq, Clone)]
enum Position {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl From<char> for Position {
    fn from(c: char) -> Self {
        match c {
            'L' => Position::EmptySeat,
            '.' => Position::Floor,
            _ => unreachable!(),
        }
    }
}

fn adjacent_occupied(room: &[Vec<Position>], r: usize, c: usize) -> u8 {
    let mut count = 0;
    for i in 1..=r.min(c) {
        if room[r - i][c - i] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r - i][c - i] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..=r {
        if room[r - i][c] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r - i][c] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..=r.min(room[0].len() - c - 1) {
        if room[r - i][c + i] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r - i][c + i] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..=c {
        if room[r][c - i] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r][c - i] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..(room[0].len() - c) {
        if room[r][c + i] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r][c + i] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..=(room.len() - r - 1).min(c) {
        if room[r + i][c - i] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r + i][c - i] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..(room.len() - r) {
        if room[r + i][c] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r + i][c] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    for i in 1..(room.len() - r).min(room[0].len() - c) {
        if room[r + i][c + i] == Position::OccupiedSeat {
            count += 1;
            break;
        }
        if room[r + i][c + i] == Position::EmptySeat || DIRECT_ONLY {
            break;
        }
    }
    count
}

fn main() {
    let mut room = INPUT.lines().map(|line| line.chars().map(Position::from).collect::<Vec<_>>()).collect::<Vec<_>>();

    loop {
        let current = room.clone();

        for r in 0..room.len() {
            for c in 0..room[r].len() {
                if room[r][c] == Position::Floor {
                    continue;
                }
                let adjacent = adjacent_occupied(&current, r, c);
                if adjacent == 0 {
                    room[r][c] = Position::OccupiedSeat;
                } else if adjacent >= TOLERANCE {
                    room[r][c] = Position::EmptySeat;
                }
            }
        }

        if room == current {
            break;
        }
    }

    let count = room.iter().map(|r| r.iter().filter(|p| **p == Position::OccupiedSeat).count()).sum::<usize>();
    println!("{}", count);
}
