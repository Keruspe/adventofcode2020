static INPUT: &str = include_str!("./11.txt");

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
    if r > 0 {
        if c > 0 {
            if room[r - 1][c - 1] == Position::OccupiedSeat {
                count += 1;
            }
        }
        if room[r - 1][c] == Position::OccupiedSeat {
            count += 1;
        }
        if c < room[r - 1].len() - 1 {
            if room[r - 1][c + 1] == Position::OccupiedSeat {
                count += 1;
            }
        }
    }
    if c > 0 {
        if room[r][c - 1] == Position::OccupiedSeat {
            count += 1;
        }
    }
    if c < room[r].len() - 1 {
        if room[r][c + 1] == Position::OccupiedSeat {
            count += 1;
        }
    }
    if r < room.len() - 1 {
        if c > 0 {
            if room[r + 1][c - 1] == Position::OccupiedSeat {
                count += 1;
            }
        }
        if room[r + 1][c] == Position::OccupiedSeat {
            count += 1;
        }
        if c < room[r + 1].len() - 1 {
            if room[r + 1][c + 1] == Position::OccupiedSeat {
                count += 1;
            }
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
                } else if adjacent >= 4 {
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
