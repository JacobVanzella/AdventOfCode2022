fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> String {
    let input = include_str!("input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let cols = 9;

    for _ in 0..cols {
        stacks.push(Vec::new());
    }

    for row in input[0].split("\n") {
        let mut row = row.chars();
        row.next();

        let mut col = 0;
        for id in row.step_by(4) {
            if !id.is_whitespace() && !id.is_digit(10) {
                stacks[col].insert(0, id);
            }
            col = (col + 1) % cols;
        }
    }

    for instruction in input[1].trim().split("\n") {
        let instruction = instruction.split(" ").collect::<Vec<&str>>();
        for _ in 0..instruction[1].parse().unwrap() {
            let id = stacks[instruction[3].parse::<usize>().unwrap() - 1].pop().unwrap();
            stacks[instruction[5].parse::<usize>().unwrap() - 1].push(id);
        }
    }

    let mut result: String = String::new();
    for mut col in stacks {
        result.push(col.pop().unwrap());
    }

    return result;
}

fn part_2() -> String {
    let input = include_str!("input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let cols = 9;

    for _ in 0..cols {
        stacks.push(Vec::new());
    }

    for row in input[0].split("\n") {
        let mut row = row.chars();
        row.next();

        let mut col = 0;
        for id in row.step_by(4) {
            if !id.is_whitespace() && !id.is_digit(10) {
                stacks[col].insert(0, id);
            }
            col = (col + 1) % cols;
        }
    }

    for instruction in input[1].trim().split("\n") {
        let instruction = instruction.split(" ").collect::<Vec<&str>>();
        let mut crane_arm: Vec<char> = Vec::new();
        for _ in 0..instruction[1].parse().unwrap() {
            let id = stacks[instruction[3].parse::<usize>().unwrap() - 1].pop().unwrap();
            crane_arm.push(id);
        }
        for id in crane_arm.iter().rev() {
            stacks[instruction[5].parse::<usize>().unwrap() - 1].push(*id);
        }
    }

    let mut result: String = String::new();
    for mut col in stacks {
        result.push(col.pop().unwrap());
    }

    return result;
}
