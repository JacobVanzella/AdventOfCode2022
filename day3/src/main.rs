use std::collections::HashMap;

fn main() {
    println!("Part 1: The sum of misplaced items is {}.", part_1());
    println!("Part 2: The sum of badge IDs is {}.", part_2());
}

fn part_1() -> i32 {
    let input = include_str!("input.txt").split("\n");
    let mut sum = 0;
    let mut count;
    let mut item_hash: HashMap<char, bool> = HashMap::new();

    for sack in input {
        count = 0;
        item_hash.clear();

        for item in sack.chars() {
            if count < sack.len() / 2 {
                item_hash.insert(item, true);
                count += 1;
            } else {
                if item_hash.contains_key(&item) {
                    sum += match item {
                        'a'..='z' => item as i32 - 96i32,
                        'A'..='Z' => item as i32 - 38i32,
                        _ => panic!("Character [{}] is missing.", item),
                    };
                    break;
                }
            }
        }
    }

    return sum;
}

fn part_2() -> i32 {
    let input = include_str!("input.txt").split("\n");
    let mut sum = 0;
    let mut group_member = 0;
    // Value is the number of sacks the key appears in.
    let mut item_hash: HashMap<char, (i32, i32)> = HashMap::new();

    for sack in input {
        group_member += 1;
        for item in sack.chars() {
            match item_hash.get(&item) {
                Some(&pair) => {
                    let mut result = Some((0,0));
                    if pair.1 < group_member {
                        result = item_hash.insert(item, (pair.0 + 1, group_member));
                    }
                    if result.unwrap().0 + 1 == 3 {
                        sum += match item {
                            'a'..='z' => item as i32 - 96i32,
                            'A'..='Z' => item as i32 - 38i32,
                            _ => panic!("Character [{}] is missing.", item),
                        };
                        group_member = 0;
                        item_hash.clear();
                        break;
                    }
                },
                None => {
                    item_hash.insert(item, (1, group_member));
                },
            }
        }
    }

    return sum;
}
