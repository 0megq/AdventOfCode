use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let elf_calories: Vec<&str> = input.split("\n\r").collect();
    let mut top_three_calories: [i32; 3] = [0; 3];
    for elf_calorie_list in elf_calories {
        let mut elf_calorie_total = 0;

        for calorie in elf_calorie_list.lines() {
            if calorie == "" {
                continue;
            }
            elf_calorie_total += calorie.trim().parse::<i32>().unwrap();
        }

        for elf in 0..top_three_calories.len() {
            if elf_calorie_total > top_three_calories[elf] {
                top_three_calories[elf] = elf_calorie_total;
                top_three_calories.sort();
                break;
            }
        }
    }
    println!(
        "{}",
        top_three_calories[0] + top_three_calories[1] + top_three_calories[2]
    );
}
