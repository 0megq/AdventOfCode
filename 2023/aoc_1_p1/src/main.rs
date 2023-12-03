use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let nums = line.chars().filter(|c| c.is_numeric());
        sum += (nums.clone().next().unwrap().to_string() + &nums.last().unwrap().to_string())
            .parse::<i32>()
            .unwrap();
    }
    println!("{}", sum);
}
