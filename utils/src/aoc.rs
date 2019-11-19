use colored::Colorize;
use std::fmt::Debug;
use std::fs;

pub struct Problem<'a, I, O> {
    pub input_file: &'a str,
    pub parser: fn(&str) -> I,
    pub part1_examples: Vec<(&'a str, O)>,
    pub part1: fn(I) -> O,
    pub part2_examples: Vec<(&'a str, O)>,
    pub part2: fn(I) -> O,
}

impl<I, O> Problem<'_, I, O>
where
    I: Debug,
    O: Debug + Eq,
{
    pub fn run(&self) {
        let input_text = read_file(self.input_file);

        println!("{}", "Part 1");
        run_examples(self.part1, self.parser, &self.part1_examples);
        run_input(self.part1, (self.parser)(&input_text));

        println!("{}", "Part 2");
        run_examples(self.part2, self.parser, &self.part2_examples);
        run_input(self.part2, (self.parser)(&input_text));
    }
}

fn read_file(file: &str) -> String {
    let s = fs::read_to_string(file).expect("Could not read file");
    s.trim().to_string()
}

fn run_examples<F, P, I, O>(f: F, parse: P, inputs: &[(&str, O)])
where
    F: Fn(I) -> O,
    P: Fn(&str) -> I,
    I: Debug,
    O: Debug + Eq,
{
    println!("Running tests:");
    for input in inputs {
        let (string, out) = &input;
        let res = f(parse(string));
        let msg = if res == *out {
            "pass".green()
        } else {
            "fail".red()
        };
        println!("{:0?} == {:1?}: {:2}", res, out, msg)
    }
}

fn run_input<F, I, O>(f: F, input: I)
where
    F: FnOnce(I) -> O,
    I: Debug,
    O: Debug,
{
    println!("Running input:");
    println!("{:0?}", f(input))
}
