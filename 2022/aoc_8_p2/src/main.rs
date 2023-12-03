use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut trees: Vec<Vec<u8>> = Vec::new();

    for (row, row_id) in input.lines().zip(0..input.lines().count()) {
        trees.push(Vec::new());
        for character in row.chars() {
            trees[row_id].push(character.to_digit(10).unwrap() as u8);
        }
    }
    //For every tree check up down left and right until we can see it.

    let mut top_score: usize = 0;

    for row in 1..(trees.len() - 1) {
        for col in 1..(trees[0].len() - 1) {
            let height = trees[row][col];
            let mut score: usize = 1;

            //Check directions
            let mut visibility = 0;
            for up in (0..row).rev() {
                visibility += 1;
                if trees[up][col] >= height {
                    break;
                }
            }
            score *= visibility;

            visibility = 0;
            for down in (row + 1)..(trees.len()) {
                visibility += 1;
                if trees[down][col] >= height {
                    break;
                }
            }
            score *= visibility;

            visibility = 0;
            for left in (0..col).rev() {
                visibility += 1;
                if trees[row][left] >= height {
                    break;
                }
            }
            score *= visibility;

            visibility = 0;
            for right in (col + 1)..trees[0].len() {
                visibility += 1;
                if trees[row][right] >= height {
                    break;
                }
            }
            score *= visibility;

            top_score = cmp::max(top_score, score);
        }
    }

    println!("{}", top_score);
}
