use std::fs;

#[derive(PartialEq, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn distance_squared(&self, other: &Vec2) -> i32 {
        let x_diff = (self.x - other.x).pow(2);
        let y_diff = (self.y - other.y).pow(2);
        return x_diff + y_diff;
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut past_tail_pos: Vec<Vec2> = vec![Vec2 { x: 0, y: 0 }];

    let mut tails: [Vec2; 10] = [Vec2 { x: 0, y: 0 }; 10];

    for line in input.lines() {
        let direction = match &line[0..1] {
            "R" => Vec2 { x: 1, y: 0 },
            "L" => Vec2 { x: -1, y: 0 },
            "U" => Vec2 { x: 0, y: 1 },
            "D" => Vec2 { x: 0, y: -1 },
            _ => {
                assert!(false);
                Vec2 { x: 0, y: 0 }
            }
        };
        let units = line[2..].parse::<usize>().unwrap();

        for unit in 0..units {
            tails[0].x += direction.x;
            tails[0].y += direction.y;
            for i in 1..tails.len() {
                if tails[i - 1].distance_squared(&tails[i]) <= 2 {
                    //check if not touching diagonal, on top, or touching side
                } else {
                    if tails[i - 1].y != tails[i].y {
                        tails[i].y += (tails[i - 1].y - tails[i].y).signum();
                    }
                    if tails[i - 1].x != tails[i].x {
                        tails[i].x += (tails[i - 1].x - tails[i].x).signum();
                    }

                    if !past_tail_pos.contains(&tails[i]) && i == 9 {
                        past_tail_pos.push(tails[i].clone());
                    }
                }
            }
        }
    }
    println!("{}", past_tail_pos.len());
}
