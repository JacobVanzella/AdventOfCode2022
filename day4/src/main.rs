fn main() {
    println!("Part 1: There are {} subset assignments.", part_1());
    println!("Part 2: {}", part_2());
}

fn part_1() -> i32 {
    let pairs = include_str!("input.txt").trim().split("\n");
    let mut sum = 0;

    for pair in pairs {
        let sections: Vec<i32> = pair
            .split(&[',', '-'][..])
            .map(|section| section.parse::<i32>().unwrap_or(-1))
            .collect();

        if sections[0] <= sections[2] && sections[1] >= sections[3]
            || sections[0] >= sections[2] && sections[1] <= sections[3]
        {
            sum += 1;
        }
    }

    return sum;
}

fn part_2() -> i32 {
    let pairs = include_str!("input.txt").trim().split("\n");
    let mut sum = 0;

    for pair in pairs {
        let sections: Vec<i32> = pair
            .split(&[',', '-'][..])
            .map(|section| section.parse::<i32>().unwrap_or(-1))
            .collect();

        if sections[0] <= sections[2] && sections[1] >= sections[3]
            || sections[0] >= sections[2] && sections[1] <= sections[3]
            || sections[1] >= sections[2] && sections[0] <= sections[2]
            || sections[0] <= sections[3] && sections[1] >= sections[3]
        {
            sum += 1;
        }
    }

    return sum;
}
