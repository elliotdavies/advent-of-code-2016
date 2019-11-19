extern crate utils;

use std::collections::HashSet;

use utils::coords;
use utils::coords::Direction;
use utils::coords::Turn;

use utils::aoc;

fn main() {
    let part1_examples =
        vec![("R2, L3", 5), ("R2, R2, R2", 2), ("R5, L5, R5, R3", 12)];

    let part2_examples = vec![("R8, R4, R4, R8", 4)];

    let problem = aoc::Problem::<In, Out> {
        input_file: "input/part1.txt",
        parser,
        part1_examples,
        part1,
        part2_examples,
        part2,
    };

    problem.run()
}

type In = Vec<(Turn, i32)>;
type Out = i32;

fn parser(input: &str) -> In {
    let replaced = input.replace(" ", "");
    let split = replaced.split(",");

    let mut parsed = Vec::new();

    for s in split {
        let (d, n) = s.split_at(1);
        let turn = match d {
            "L" => Turn::L,
            _ => Turn::R,
        };
        parsed.push((turn, n.parse().unwrap()));
    }

    parsed
}

fn part1(input: In) -> Out {
    let mut coords = (0, 0);
    let mut dir = Direction::N;

    for (t, n) in input {
        dir = coords::turn(&dir, &t);
        coords = coords::step(&coords, &dir, n);
    }

    coords::manhattan(&coords)
}

fn part2(input: In) -> Out {
    let mut coords = (0, 0);
    let mut dir = Direction::N;
    let mut seen = HashSet::new();
    seen.insert(coords);

    'outer: for (t, n) in input {
        dir = coords::turn(&dir, &t);

        for _ in 0..n {
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
