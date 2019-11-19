extern crate utils;

use std::char;

use utils::coords;
use utils::coords::Direction;

use utils::input;
use utils::tdd;

fn main() {
    let input = input::read_file("input.txt");

    let test_in_1 = String::from(
        "ULL
        RRDDD
        LURDL
        UUUUD",
    );
    let test_in_2 = test_in_1.clone();

    println!("{}", "Part 1");
    let test_out_1 = String::from("1985");
    tdd::test(part1, &[(test_in_1, test_out_1)]);
    tdd::run(part1, &input);

    println!("{}", "Part 2");
    let test_out_2 = String::from("5DB3");
    tdd::test(part2, &[(test_in_2, test_out_2)]);
    tdd::run(part2, &input);
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

fn coords_to_keypad_pt1((x, y): coords::Coords) -> Option<String> {
    if x >= 0 && x < 3 && y >= 0 && y < 3 {
        // (0,0) is the bottom-left corner where '7' is on the keypad
        let key = x + 7 - (3 * y);
        Some(key.to_string())
    } else {
        None
    }
}

fn coords_to_keypad_pt2((x, y): coords::Coords) -> Option<String> {
    let convert_i32 = |i: i32| Some(i.to_string());
    let convert_str = |s: &str| Some(s.to_string());

    match y {
        4 if x == 2 => convert_i32(1),
        3 if x >= 1 && x <= 3 => convert_i32(x + 1),
        2 if x >= 0 && x <= 4 => convert_i32(x + 5),
        1 if x == 1 => convert_str("A"),
        1 if x == 2 => convert_str("B"),
        1 if x == 3 => convert_str("C"),
        0 if x == 2 => convert_str("D"),
        _ => None,
    }
}

fn go<F>(
    input: &String,
    initial_coords: coords::Coords,
    coords_to_keypad: F,
) -> String
where
    F: Fn(coords::Coords) -> Option<String>,
{
    let mut res = String::new();
    let mut coords = initial_coords;

    for line in input.lines() {
        let dirs = line.chars().filter_map(|c| char_to_dir(c));

        for dir in dirs {
            let new_coords = coords::step(&coords, &dir, 1);
            if coords_to_keypad(new_coords).is_some() {
                coords = new_coords;
            }
        }

        res.push_str(&coords_to_keypad(coords).expect("").to_string())
    }

    res
}

fn part1(input: &String) -> String {
    go(input, (1, 1), coords_to_keypad_pt1)
}

fn part2(input: &String) -> String {
    go(input, (0, 2), coords_to_keypad_pt2)
}
