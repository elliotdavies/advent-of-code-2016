extern crate utils;

use utils::input;
use utils::tdd;

fn main() {
    let input = input::read_file("input.txt");

    let tests = [
        (String::from("5 10 25"), 0),
        (String::from("3 10 12\n3 4 5"), 2),
    ];

    println!("{}", "Part 1");
    tdd::test(part1, &tests);
    tdd::run(part1, &input);

    println!("{}", "Part 2");
    tdd::run(part2, &input);
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

fn part1(input: &String) -> i32 {
    let mut num_valid = 0;

    for line in input.lines() {
        let (x, y, z) = parse_line(line);
        if valid(x, y, z) {
            num_valid += 1
        }
    }

    num_valid
}

fn part2(input: &String) -> i32 {
    let mut num_valid = 0;

    let mut lines = input.lines();
    loop {
        let l1 = lines.next();
        let l2 = lines.next();
        let l3 = lines.next();

        if (l3.is_none()) {
            break;
        }

        let (x1, y1, z1) = parse_line(l1.unwrap());
        let (x2, y2, z2) = parse_line(l2.unwrap());
        let (x3, y3, z3) = parse_line(l3.unwrap());

        if valid(x1, x2, x3) {
            num_valid += 1
        }
        if valid(y1, y2, y3) {
            num_valid += 1
        }
        if valid(z1, z2, z3) {
            num_valid += 1
        }
    }

    num_valid
}
