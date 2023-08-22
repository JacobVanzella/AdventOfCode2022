fn main() {
    println!("Part 1: Your score is {}.", part_1());
    println!("Part 2: Your score is {}.", part_2());
}

fn part_1() -> i32 {
    let input = include_str!("input.txt").split("\n");

    let mut score = 0;
    for round in input {
        score += match round {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            "" => 0,
            _ => panic!("Pattern [{}] missing!", round),
        }
    }

    return score;
}

fn part_2() -> i32 {
    let input = include_str!("input.txt").split("\n");

    let mut score = 0;
    for round in input {
        score += match round {
            "A X" => 0 + 3,
            "A Y" => 3 + 1,
            "A Z" => 6 + 2,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 0 + 2,
            "C Y" => 3 + 3,
            "C Z" => 6 + 1,
            "" => 0,
            _ => panic!("Pattern [{}] missing!", round),
        }
    }

    return score;
}
