extern crate utils;

use utils::input;
use utils::tdd;

fn main() {
    let test_inputs = 
        [ (String::from("R2, L3"),          5)
        , (String::from("R2, R2, R2"),      2)
        , (String::from("R5, L5, R5, R3"), 12)
        ];
    tdd::test(part1, &test_inputs);
    
    let input = input::read_file("input/part1.txt");
    tdd::run(part1, &input)
}

fn part1(input: &String) -> i32 {
    0 // TODO
}
