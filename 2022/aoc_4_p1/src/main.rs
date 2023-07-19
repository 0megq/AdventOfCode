use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut containing_pairs: usize = 0;
    for line in input.lines() {
        let ranges: Vec<&str> = line.split(",").collect();
        let range_a: Vec<&str> = ranges[0].split("-").collect();
        let range_b: Vec<&str> = ranges[1].split("-").collect();

        if ranges_overlapping(range_a, range_b) {
            containing_pairs += 1;
        }
    }

    println!("{}", containing_pairs);
}

fn ranges_overlapping(a: Vec<&str>, b: Vec<&str>) -> bool {
    let top_a = a[0].parse::<usize>().unwrap();
    let bottom_a = a[1].parse::<usize>().unwrap();
    let top_b = b[0].parse::<usize>().unwrap();
    let bottom_b = b[1].parse::<usize>().unwrap();

    if top_a >= top_b && bottom_a <= bottom_b {
        return true;
    } else if top_b >= top_a && bottom_b <= bottom_a {
        return true;
    } else {
        return false;
    }
}
