extern crate utils;

use std::collections::HashSet;
use utils::input;
use utils::tdd;

fn main() {
    let input = input::read_file("input/part1.txt");

    println!("{}", "Part 1");
    let test_inputs = 
        [ (String::from("R2, L3"),          5)
        , (String::from("R2, R2, R2"),      2)
        , (String::from("R5, L5, R5, R3"), 12)
        ];
    tdd::test(part1, &test_inputs);
    tdd::run(part1, &input);

    println!("{}", "Part 2");
    tdd::test(part2, &[(String::from("R8, R4, R4, R8"), 4)]);
    tdd::run(part2, &input);
}

type Coords = (i32, i32);

enum Direction {
    N,
    E,
    S,
    W
}

fn turn(dir: &Direction, s: &str) -> Direction {
    match dir {
        Direction::N => {
            if s == "L" { Direction::W }
            else        { Direction::E }
        }
        Direction::E => {
            if s == "L" { Direction::N }
            else        { Direction::S }
        }
        Direction::S => {
            if s == "L" { Direction::E }
            else        { Direction::W }
        }
        Direction::W => {
            if s == "L" { Direction::S }
            else        { Direction::N }
        }
    }
}

fn step((x,y): &Coords, dir: &Direction, n: i32) -> Coords {
    match dir {
        Direction::N => (*x, y+n),
        Direction::E => (x+n, *y),
        Direction::S => (*x, y-n),
        Direction::W => (x-n, *y)
    }
}

fn manhattan((x,y): &Coords) -> i32 {
    x.abs() + y.abs()
}

fn part1(input: &String) -> i32 {
    let replaced = input.replace(" ", "");
    let split = replaced.split(",");

    let mut coords = (0, 0);
    let mut dir = Direction::N;

    for s in split {
        let (d,n) = s.split_at(1);
        dir = turn(&dir, d);
        coords = step(&coords, &dir, n.parse().unwrap())
    }
    manhattan(&coords)
}

fn part2(input: &String) -> i32 {
    let replaced = input.replace(" ", "");
    let split = replaced.split(",");

    let mut coords = (0, 0);
    let mut dir = Direction::N;
    let mut seen = HashSet::new();
    seen.insert(coords);

    'outer: for s in split {
        let (d,n) = s.split_at(1);
        dir = turn(&dir, d);

        for _ in 0..(n.parse().unwrap()) {
            coords = step(&coords, &dir, 1);

            if seen.contains(&coords) { break 'outer; }
            else { seen.insert(coords); }
        }
    }
    manhattan(&coords)
}
