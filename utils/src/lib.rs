pub mod input {
    use std::fs;

    pub fn read_file(file: &str) -> String {
        let s = fs::read_to_string(file).expect("Could not read file");
        s.trim().to_string()
    }
}

pub mod tdd {
    use std::fmt::Debug;
    use colored::Colorize;

    pub fn test<F, I, O>(f: F, inputs: &[(I, O)])
    where
        F: Fn(&I) -> O,
        I: Debug,
        O: Debug + Eq,
    {
        println!("Running tests:");
        for input in inputs {
            let (i, o) = &input;
            let res = f(i);
            let msg = if res == *o { "pass".green() } else { "fail".red() };
            println!("{:0?} == {:1?}: {:2}", res, o, msg)
        }
    }

    pub fn run<F, I, O>(f: F, input: &I)
    where
        F: FnOnce(&I) -> O,
        I: Debug,
        O: Debug,
    {
        println!("Running input:");
        println!("{:0?}", f(input))
    }
}

pub mod coords {
    pub type Coords = (i32, i32);

    pub enum Direction {
        N,
        E,
        S,
        W,
    }

    pub fn turn(dir: &Direction, s: &str) -> Direction {
        match dir {
            Direction::N => {
                if s == "L" {
                    Direction::W
                } else {
                    Direction::E
                }
            }
            Direction::E => {
                if s == "L" {
                    Direction::N
                } else {
                    Direction::S
                }
            }
            Direction::S => {
                if s == "L" {
                    Direction::E
                } else {
                    Direction::W
                }
            }
            Direction::W => {
                if s == "L" {
                    Direction::S
                } else {
                    Direction::N
                }
            }
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
