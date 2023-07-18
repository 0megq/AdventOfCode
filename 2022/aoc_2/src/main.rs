use std::fs;

enum Points {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

const OPPONENT_INPUT: [char; 3] = ['A', 'B', 'C'];
const PLAYER_INPUT: [char; 3] = ['X', 'Y', 'Z'];
const CHOICE_POINTS: [u8; 3] = [1, 2, 3];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_points: usize = 0;

    for line in input.lines() {
        let opponent_strength = get_input_strength(&line[0..1], OPPONENT_INPUT).unwrap();
        let player_strength = get_input_strength(&line[2..3], PLAYER_INPUT).unwrap();

        let points = calculate_points(player_strength, opponent_strength) as usize;
        total_points += points
    }

    println!("{}", total_points)
}

fn calculate_points(player_strength: usize, opponent_strength: usize) -> u8 {
    let match_points = if player_strength == (opponent_strength + 1) % 3 {
        Points::Win
    } else if (player_strength + 1) % 3 == opponent_strength {
        Points::Loss
    } else {
        Points::Draw
    };

    match_points as u8 + CHOICE_POINTS[player_strength]
}

fn get_input_strength(input: &str, input_array: [char; 3]) -> Option<usize> {
    for item_id in 0..input_array.len() {
        if input.parse::<char>().unwrap() == input_array[item_id] {
            return Some(item_id);
        }
    }
    return None;
}
