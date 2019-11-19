pub type Coords = (i32, i32);

#[derive(Debug)]
pub enum Turn {
    L,
    R,
}

#[derive(Debug)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

pub fn turn(dir: &Direction, turn: &Turn) -> Direction {
    match (dir, turn) {
        (Direction::N, Turn::L) => Direction::W,
        (Direction::N, Turn::R) => Direction::E,
        (Direction::E, Turn::L) => Direction::N,
        (Direction::E, Turn::R) => Direction::S,
        (Direction::S, Turn::L) => Direction::E,
        (Direction::S, Turn::R) => Direction::W,
        (Direction::W, Turn::L) => Direction::S,
        (Direction::W, Turn::R) => Direction::N,
    }
}

pub fn step((x, y): &Coords, dir: &Direction, n: i32) -> Coords {
    match dir {
        Direction::N => (*x, y + n),
        Direction::E => (x + n, *y),
        Direction::S => (*x, y - n),
        Direction::W => (x - n, *y),
    }
}

pub fn manhattan((x, y): &Coords) -> i32 {
    x.abs() + y.abs()
}
