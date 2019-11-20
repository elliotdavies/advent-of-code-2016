extern crate utils;

use utils::aoc;

fn main() {
    let part1_examples = vec![];
    let part2_examples = vec![];

    let problem = aoc::Problem::<In, Out> {
        input_file: "src/day$DAY.txt",
        parser,
        part1_examples,
        part1,
        part2_examples,
        part2,
    };

    problem.run()
}

type In = ??;
type Out = ??;

fn parser(input: &str) -> In {
}

fn part1(input: In) -> Out {
}

fn part2(input: In) -> Out {
}

