use std::fs;

#[derive(PartialEq)]
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

    let mut head_pos: Vec2 = Vec2 { x: 0, y: 0 };
    let mut tail_pos: Vec2 = Vec2 { x: 0, y: 0 };

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
        println!("{}", line);
        for unit in 0..units {
            head_pos.x += direction.x;
            head_pos.y += direction.y;
            if head_pos.distance_squared(&tail_pos) <= 2 {
                //check if not touching diagonal, on top, or touching side
            } else {
                if head_pos.y != tail_pos.y {
                    tail_pos.y += (head_pos.y - tail_pos.y).signum();
                }
                if head_pos.x != tail_pos.x {
                    tail_pos.x += (head_pos.x - tail_pos.x).signum();
                }

                if !past_tail_pos.contains(&tail_pos) {
                    past_tail_pos.push(Vec2 {
                        //budget clone()
                        x: tail_pos.x,
                        y: tail_pos.y,
                    });

                    println!("x: {} y: {}", tail_pos.x, tail_pos.y);
                }
            }
        }
    }
    println!("{}", past_tail_pos.len());
}
