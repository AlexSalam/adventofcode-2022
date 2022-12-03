use crate::shared::read_lines;

pub fn count_duplicate_priorities()
{
    if let Ok(lines) = read_lines("./src/data/3/data.txt") {
        let mut priority = 0;
        let mut grouped_priority = 0;
        let placeholder = String::from("");
        let mut group: [String; 3] = [placeholder.clone(), placeholder.clone(), placeholder.clone()];
        for (i, line) in lines.enumerate() {
            if let Ok(backpacks) = line {

                // Handle backpack groups (part 2)
                group[i % 3] = backpacks.clone();
                if !group.contains(&placeholder) {
                    grouped_priority = grouped_priority + find_common_item_and_prioritise(group);
                    group = [placeholder.clone(), placeholder.clone(), placeholder.clone()];
                }

                // Get the packs as strings
                let (pack_one, pack_two) = backpacks.split_at(backpacks.len() / 2);
                // Convert them into Vectors
                let pack_one: Vec<char> = pack_one.chars().collect();
                let pack_two: Vec<char> = pack_two.chars().collect();

                for item in pack_one.iter() {
                    if pack_two.contains(item) {
                        priority = priority + get_priority(&item);
                        break;
                    }
                }
            }
        }
        println!("Total priority: {priority}");
        println!("Total grouped priority: {grouped_priority}");
    }
}

fn find_common_item_and_prioritise(group: [String; 3]) -> i32
{
    let mut common_items = Vec::new();
    let pack_one: Vec<char> = group[0].chars().collect();
    let pack_two: Vec<char> = group[1].chars().collect();
    let pack_three: Vec<char> = group[2].chars().collect();

    for item in pack_one.iter() {
        if pack_two.contains(item) && !common_items.contains(item) {
            common_items.push(*item);
        }
    }
    for item in common_items.iter() {
        if pack_three.contains(item) {
            return get_priority(item);
        }
    }
    0
}

fn get_priority(item: &char) -> i32
{
    let mut count = 0;
    for c in 'a' ..= 'z' {
        count = count + 1;
        if c == *item {
            return count;
        }
    }
    for c in 'A' ..= 'Z' {
        count = count + 1;
        if c == *item {
            return count;
        }
    }
    0
}
