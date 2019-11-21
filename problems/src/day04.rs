extern crate utils;

use utils::aoc;

use std::collections::HashMap;

fn main() {
    let part1_examples = vec![
        ("aaaaa-bbb-z-y-x-123[abxyz]", 123),
        (
            "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]",
            1514,
        ),
    ];
    let part2_examples = vec![("qzmt-zixmtkozy-ivhz-343[zimth]", 0)];

    let problem = aoc::Problem::<In, Out> {
        input_file: "src/day04.txt",
        parser,
        part1_examples,
        part1,
        part2_examples,
        part2,
    };

    problem.run()
}

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    freqs: HashMap<char, i32>,
    id: i32,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut key_vals: Vec<(&char, &i32)> = self.freqs.iter().collect();
        key_vals.sort_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }
        });
        let (most_common, _) = key_vals.split_at(5);
        most_common.iter().map(|(k, _)| *k).collect::<String>() == self.checksum
    }

    fn decrypt(&self) -> String {
        fn shift(c: char, n: i32) -> char {
            let shift = c as u32 - b'a' as u32 + n as u32;
            ((shift % 26) as u8 + b'a') as char
        }

        self.encrypted_name
            .chars()
            .map(|c| if c == '-' { ' ' } else { shift(c, self.id) })
            .collect()
    }
}

type In = Vec<Room>;
type Out = i32;

fn parser(input: &str) -> In {
    let mut rooms = Vec::new();
    for line in input.lines() {
        let mut encrypted_name = String::new();
        let mut freqs = HashMap::new();
        let mut id = 0;
        let mut checksum = String::new();

        for chunk in line.split('-') {
            match chunk.chars().nth(0) {
                Some(first) if first.is_ascii_alphabetic() => {
                    // One of the letter groups
                    encrypted_name.push_str(chunk);
                    encrypted_name.push('-');
                    for c in chunk.chars() {
                        *freqs.entry(c).or_insert(0) += 1;
                    }
                }
                _ => {
                    // Tail
                    id = chunk
                        .chars()
                        .take_while(|x| x.is_ascii_digit())
                        .collect::<String>()
                        .to_string()
                        .parse()
                        .unwrap();

                    checksum = chunk
                        .chars()
                        .skip_while(|x| x.is_ascii_digit())
                        .skip(1)
                        .take_while(|x| x.is_ascii_alphabetic())
                        .collect::<String>();

                    break;
                }
            }
        }
        rooms.push(Room {
            encrypted_name,
            freqs,
            id,
            checksum: String::from(checksum),
        });
    }
    rooms
}

fn part1(input: In) -> Out {
    input
        .iter()
        .filter(|room| room.is_valid())
        .fold(0, |acc, room| acc + room.id)
}

fn part2(input: In) -> Out {
    input
        .iter()
        .filter(|room| room.is_valid())
        .map(|room| (room.decrypt(), room.id))
        .filter(|(name, _)| name.contains("north"))
        .for_each(|(name, id)| println!("{0} {1}", name, id));

    0
}
