use std::fs;
use std::iter::zip;

const PRIORITIES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut triplicate_list = Vec::new();

    let mut input = input.lines();
    for _line_index in 0..input.clone().count() / 3 {
        let first_bag = input.next().unwrap();
        let second_bag = input.next().unwrap();
        let third_bag = input.next().unwrap();

        'first_bag: for character_a in first_bag.chars() {
            for character_b in second_bag.chars() {
                for character_c in third_bag.chars() {
                    if character_a == character_b && character_b == character_c {
                        triplicate_list.push(character_a);
                        break 'first_bag;
                    }
                }
            }
        }
    }

    let mut total_priority = 0;
    for character in triplicate_list {
        for (letter, priority) in zip(PRIORITIES.chars(), 1..PRIORITIES.len() + 1) {
            if letter == character {
                total_priority += priority;
            }
        }
    }

    println!("{}", total_priority)
}
