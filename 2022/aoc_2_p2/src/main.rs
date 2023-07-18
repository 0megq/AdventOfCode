use std::fs;

const OPPONENT_INPUT: [char; 3] = ['A', 'B', 'C'];
const MATCH_STATUS_OPTIONS: [char; 3] = ['X', 'Y', 'Z'];
const CHOICE_POINTS: [u8; 3] = [1, 2, 3];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut total_points: usize = 0;

    for line in input.lines() {
        let opponent_strength = get_input_strength(&line[0..1], OPPONENT_INPUT).unwrap();
        let match_status_points = get_match_status_points(&line[2..3]).unwrap();

        let points = calculate_points(match_status_points, opponent_strength) as usize;
        total_points += points
    }

    println!("{}", total_points)
}

fn calculate_points(match_status_points: usize, opponent_strength: usize) -> u8 {
    let player_strength = if match_status_points == 0 {
        (opponent_strength + 2) % 3
    } else if match_status_points == 3 {
        opponent_strength
    } else {
        (opponent_strength + 1) % 3
    };
    match_status_points as u8 + CHOICE_POINTS[player_strength]
}

fn get_match_status_points(input: &str) -> Option<usize> {
    for status_id in 0..MATCH_STATUS_OPTIONS.len() {
        if input.parse::<char>().unwrap() == MATCH_STATUS_OPTIONS[status_id] {
            return Some(status_id * 3);
        }
    }

    return None;
}

fn get_input_strength(input: &str, input_array: [char; 3]) -> Option<usize> {
    for item_id in 0..input_array.len() {
        if input.parse::<char>().unwrap() == input_array[item_id] {
            return Some(item_id);
        }
    }

    return None;
}
