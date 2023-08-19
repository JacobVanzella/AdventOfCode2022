use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    println!("The elf is hording {} calories.", part_1());
    println!("The top three elfs are hording {} calories.", part_2());
}

fn part_1() -> i32 {
    let input = include_str!("input.txt").split("\n\n");
    let mut max_cal = 0;
    let mut cal_count;

    for sack in input {
        cal_count = 0;
        for cal in sack.split("\n") {
            cal_count += cal.parse::<i32>().unwrap_or(0);
        }

        if cal_count > max_cal {
            max_cal = cal_count;
        }
    }

    return max_cal;
}

fn part_2() -> i32 {
    let input = include_str!("input.txt").split("\n\n");
    let mut max_cal = BinaryHeap::with_capacity(3);
    let mut cal_count;

    for sack in input {
        cal_count = 0;
        for cal in sack.split("\n") {
            cal_count += cal.parse::<i32>().unwrap_or(0);
        }

        if max_cal.len() < 3 {
            max_cal.push(Reverse(cal_count));
        }
        else if cal_count > max_cal.peek().unwrap().0 {
            max_cal.pop();
            max_cal.push(Reverse(cal_count));
        }
    }

    return max_cal.iter().map(|rev| rev.0).sum();
}
