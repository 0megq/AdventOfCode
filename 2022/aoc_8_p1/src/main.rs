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

    println!("{:?}", trees);
    //For every tree check up down left and right until we can see it.

    let mut visible_trees: usize = 0;

    for row in 0..trees.len() {
        for col in 0..trees[0].len() {
            let height = trees[row][col];
            //Check up
            let mut visible = true;
            for up in 0..row {
                if trees[up][col] >= height {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            }

            visible = true;

            for down in (row + 1)..(trees.len()) {
                if trees[down][col] >= height {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            }

            visible = true;

            for left in 0..col {
                if trees[row][left] >= height {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            }

            visible = true;

            for right in (col + 1)..trees[0].len() {
                if trees[row][right] >= height {
                    visible = false;
                    break;
                }
            }

            if visible {
                visible_trees += 1;
                continue;
            }
        }
    }

    println!("{}", visible_trees);
}
