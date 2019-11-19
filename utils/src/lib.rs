pub mod aoc {
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
            tdd::test(self.part1, self.parser, &self.part1_examples);
            tdd::run(self.part1, (self.parser)(&input_text));

            println!("{}", "Part 2");
            tdd::test(self.part2, self.parser, &self.part2_examples);
            tdd::run(self.part2, (self.parser)(&input_text));
        }
    }

    pub fn read_file(file: &str) -> String {
        let s = fs::read_to_string(file).expect("Could not read file");
        s.trim().to_string()
    }

    mod tdd {
        use colored::Colorize;
        use std::fmt::Debug;

        pub fn test<F, P, I, O>(f: F, p: P, inputs: &[(&str, O)])
        where
            F: Fn(I) -> O,
            P: Fn(&str) -> I,
            I: Debug,
            O: Debug + Eq,
        {
            println!("Running tests:");
            for input in inputs {
                let (s, o) = &input;
                let i = p(s);
                let res = f(i);
                let msg = if res == *o {
                    "pass".green()
                } else {
                    "fail".red()
                };
                println!("{:0?} == {:1?}: {:2}", res, o, msg)
            }
        }

        pub fn run<F, I, O>(f: F, input: I)
        where
            F: FnOnce(I) -> O,
            I: Debug,
            O: Debug,
        {
            println!("Running input:");
            println!("{:0?}", f(input))
        }
    }
}

pub mod coords {
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
}
