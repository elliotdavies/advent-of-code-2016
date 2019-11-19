extern crate utils;

use utils::aoc;

fn main() {
    let part1_examples = vec![("5 10 25", 0), ("3 10 12\n3 4 5", 2)];

    let problem = aoc::Problem::<In, Out> {
        input_file: "input.txt",
        parser,
        part1_examples,
        part1,
        part2_examples: vec![],
        part2,
    };

    problem.run()
}

type In = Vec<(i32, i32, i32)>;
type Out = i32;

fn parser(input: &str) -> In {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> (i32, i32, i32) {
    let mut xs = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let x: i32 = xs.next().unwrap();
    let y: i32 = xs.next().unwrap();
    let z: i32 = xs.next().unwrap();
    (x, y, z)
}

fn valid(x: i32, y: i32, z: i32) -> bool {
    x + y > z && x + z > y && y + z > x
}

fn part1(input: In) -> Out {
    input.iter().fold(
        0,
        |acc, (x, y, z)| if valid(*x, *y, *z) { acc + 1 } else { acc },
    )
}

fn part2(input: In) -> Out {
    let mut new_input = Vec::new();

    for chunk in input.chunks_exact(3) {
        let (x1, y1, z1) = chunk[0];
        let (x2, y2, z2) = chunk[1];
        let (x3, y3, z3) = chunk[2];

        new_input.push((x1, x2, x3));
        new_input.push((y1, y2, y3));
        new_input.push((z1, z2, z3));
    }

    part1(new_input)
}
