use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let input: Vec<&str> = input.split("\n\r").collect();

    let stack_input = input[0];
    let mut stacks: Vec<Vec<char>> = Vec::new();

    let instruction_input: &str = input[1];

    for (line, line_id) in stack_input.lines().zip(0..stack_input.lines().count()) {
        if line_id >= stack_input.lines().count() - 1 {
            break;
        }

        let stack_count = (line.len() + 1) / 4;

        for stack in 0..stack_count {
            if line_id == 0 {
                stacks.push(Vec::new());
            }

            let current_crate = &line[(1 + stack * 4)..(2 + stack * 4)];
            if current_crate != " " {
                stacks[stack].push(current_crate.parse::<char>().unwrap());
            }
        }
    }

    for (instruction, instruction_id) in instruction_input
        .lines()
        .zip(0..instruction_input.lines().count())
    {
        if instruction_id == 0 {
            continue;
        }

        let extra_digits = instruction.len() - 18;

        let crate_amount = &instruction[5..6 + extra_digits];
        let from_stack_index = &instruction[12 + extra_digits..13 + extra_digits];
        let to_stack_index = &instruction[17 + extra_digits..18 + extra_digits];

        let crate_amount = crate_amount.parse::<usize>().unwrap();
        let from_stack_index = from_stack_index.parse::<usize>().unwrap() - 1;
        let to_stack_index = to_stack_index.parse::<usize>().unwrap() - 1;

        let from_stack = &stacks[from_stack_index].clone();
        let to_stack = &mut stacks[to_stack_index];

        //For part 1 remove the .rev() vvv
        for crate_id in (0..crate_amount).rev() {
            to_stack.insert(0, from_stack[crate_id]);
        }

        let from_stack = &mut stacks[from_stack_index];

        for crate_id in 0..crate_amount {
            from_stack.remove(0);
        }
    }

    for stack in stacks {
        print!("{}", stack[0]);
    }
}
