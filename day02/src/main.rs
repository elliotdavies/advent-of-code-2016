extern crate utils;

use std::char;

use utils::coords;
use utils::coords::Direction;

use utils::aoc;

fn main() {
    let example_input = "ULL
        RRDDD
        LURDL
        UUUUD";

    let problem = aoc::Problem::<In, Out> {
        input_file: "input.txt",
        parser,
        part1_examples: vec![(example_input, "1985".to_string())],
        part1,
        part2_examples: vec![(example_input, "5DB3".to_string())],
        part2,
    };

    problem.run()
}

type In = Vec<Vec<Direction>>;
type Out = String;

fn parser(input: &str) -> In {
    let char_to_dir = |c| match c {
        'U' => Some(Direction::N),
        'R' => Some(Direction::E),
        'D' => Some(Direction::S),
        'L' => Some(Direction::W),
        _ => None,
    };

    let mut outer = Vec::new();
    for line in input.lines() {
        let mut inner = Vec::new();
        let dirs = line.chars().filter_map(|c| char_to_dir(c));
        for dir in dirs {
            inner.push(dir);
        }
        outer.push(inner);
    }
    outer
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

fn part1(input: In) -> Out {
    let mut res = String::new();
    let mut coords = (1, 1);

    for line in input {
        for dir in line {
            let new_coords = coords::step(&coords, &dir, 1);
            if coords_to_keypad_pt1(new_coords).is_some() {
                coords = new_coords;
            }
        }

        res.push_str(&coords_to_keypad_pt1(coords).expect("").to_string())
    }

    res
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

fn part2(input: In) -> Out {
    let mut res = String::new();
    let mut coords = (0, 2);

    for line in input {
        for dir in line {
            let new_coords = coords::step(&coords, &dir, 1);
            if coords_to_keypad_pt2(new_coords).is_some() {
                coords = new_coords;
            }
        }

        res.push_str(&coords_to_keypad_pt2(coords).expect("").to_string())
    }

    res
}
