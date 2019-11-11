pub mod input {
    use std::fs;

    pub fn read_file(file: &str) -> String {
        fs::read_to_string(file).expect("Could not read file")
    }
}

pub mod tdd {
    use std::fmt::Display;

    pub fn test<F, I, O>(f: F, inputs: &[(I, O)]) where
        F: Fn(&I) -> O,
        I: Display,
        O: Display
    {
        println!("Running tests:");
        for input in inputs {
            let (i, o) = &input;
            println!("{0} == {1}", f(i), o)
        }
    }

    pub fn run<F, I, O>(f: F, input: &I) where
        F: FnOnce(&I) -> O,
        I: Display,
        O: Display
    {
        println!("Running input:");
        println!("{0}", f(input))
    }
}
