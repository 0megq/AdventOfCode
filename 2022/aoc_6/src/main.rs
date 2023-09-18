use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    'outer: for index in 3..input.len() - 1 {
        for (a, index_a) in input[index - 3..index + 1].chars().zip(0..4) {
            for (b, index_b) in input[index - 3..index + 1].chars().zip(0..4) {
                if index_a == index_b {
                    continue;
                }

                if a == b {
                    continue 'outer;
                }
            }
        }

        println!("{}", index + 1);
        break;
    }
}
