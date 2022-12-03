use crate::shared::read_lines;

pub fn count_duplicate_priorities()
{
    if let Ok(lines) = read_lines("./src/data/3/data.txt") {
        let mut priority = 0;
        for line in lines {
            if let Ok(backpacks) = line {
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
        println!("Total priority: {priority}")
    }
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
