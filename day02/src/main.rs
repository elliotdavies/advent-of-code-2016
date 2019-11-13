extern crate utils;

use utils::coords;
use utils::coords::Direction;

use utils::input;
use utils::tdd;

fn main() {
    let input = input::read_file("input.txt");

    println!("{}", "Part 1");
    let test_in = String::from(
        "ULL
        RRDDD
        LURDL
        UUUUD",
    );
    let test_out = String::from("1985");
    tdd::test(part1, &[(test_in, test_out)]);
    tdd::run(part1, &input);
}

fn char_to_dir(c: char) -> Option<Direction> {
    match c {
        'U' => Some(Direction::N),
        'R' => Some(Direction::E),
        'D' => Some(Direction::S),
        'L' => Some(Direction::W),
        _ => None,
    }
}

fn coords_to_num((x,y): coords::Coords) -> i32 {
    x + 7 - (3 * y) // (0,0) is the bottom-left corner where '7' is on the keypad
}

fn part1(input: &String) -> String {
    let mut res = String::new();
    let mut coords = (1, 1);

    for line in input.lines() {
        let mut dir = Direction::N;

        let dirs = line.chars().filter_map(|c| char_to_dir(c));

        for d in dirs {
            dir = d;
            let (x, y) = coords::step(&coords, &dir, 1);
            if x >= 0 && x < 3 && y >= 0 && y < 3 {
                coords = (x, y);
            }
        }

        res.push_str(&coords_to_num(coords).to_string())
    }

    res
}
