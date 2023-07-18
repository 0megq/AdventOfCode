use std::fs;
use std::iter::zip;

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut duplicate_list = Vec::new();
    for line in input.lines() {
        let first_half = &line[..line.len() / 2];
        let second_half = &line[line.len() / 2..];

        'first_half: for character_a in first_half.chars() {
            for character_b in second_half.chars() {
                if character_a == character_b {
                    duplicate_list.push(character_a);
                    break 'first_half;
                }
            }
        }
    }

    let mut total_priority: usize = 0;
    for character in duplicate_list {
        for (letter, priority) in zip(PRIORITIES.chars(), 1..PRIORITIES.len() + 1) {
            if letter == character {
                total_priority += priority;
            }
        }
    }

    println!("{}", total_priority)
}
