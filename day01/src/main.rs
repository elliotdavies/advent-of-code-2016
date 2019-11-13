extern crate utils;

use std::collections::HashSet;

use utils::coords;
use utils::coords::Direction;

use utils::input;
use utils::tdd;

fn main() {
    let input = input::read_file("input/part1.txt");

    println!("{}", "Part 1");
    let test_inputs = [
        (String::from("R2, L3"), 5),
        (String::from("R2, R2, R2"), 2),
        (String::from("R5, L5, R5, R3"), 12),
    ];
    tdd::test(part1, &test_inputs);
    tdd::run(part1, &input);

    println!("{}", "Part 2");
    tdd::test(part2, &[(String::from("R8, R4, R4, R8"), 4)]);
    tdd::run(part2, &input);
}

fn part1(input: &String) -> i32 {
    let replaced = input.replace(" ", "");
    let split = replaced.split(",");

    let mut coords = (0, 0);
    let mut dir = Direction::N;

    for s in split {
        let (d, n) = s.split_at(1);
        dir = coords::turn(&dir, d);
        coords = coords::step(&coords, &dir, n.parse().unwrap())
    }
    coords::manhattan(&coords)
}

fn part2(input: &String) -> i32 {
    let replaced = input.replace(" ", "");
    let split = replaced.split(",");

    let mut coords = (0, 0);
    let mut dir = Direction::N;
    let mut seen = HashSet::new();
    seen.insert(coords);

    'outer: for s in split {
        let (d, n) = s.split_at(1);
        dir = coords::turn(&dir, d);

        for _ in 0..(n.parse().unwrap()) {
            coords = coords::step(&coords, &dir, 1);

            if seen.contains(&coords) {
                break 'outer;
            } else {
                seen.insert(coords);
            }
        }
    }
    coords::manhattan(&coords)
}
