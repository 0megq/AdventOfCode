use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    'outer: for index in 14..input.len() - 1 {
        for (a, index_a) in input[index - 14..index].chars().zip(0..14) {
            for (b, index_b) in input[index - 14..index].chars().zip(0..14) {
                if index_a == index_b {
                    continue;
                }

                if a == b {
                    continue 'outer;
                }
            }
        }

        println!("{} {}", index, &input[index - 14..index]);
        break;
    }
}
