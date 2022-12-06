use crate::shared::read_lines;

pub fn get_first_marker()
{
    if let Ok(mut lines) = read_lines("./src/data/6/data.txt") {
        if let Some(line) = lines.next() {
            if let Ok(input) = line {
                let mut count = 0;
                let chars: Vec<char> = input.chars().collect();
                let mut marker: [char; 4] = ['A'; 4];
                let mut message: [char; 14] = ['A'; 14];
                for char in chars.into_iter() {
                    marker.rotate_left(1);
                    message.rotate_left(1);
                    marker[3] = char;
                    message[13] = char;
                    count = count + 1;
                    if validate_marker(&marker) {
                        println!("Marker found at position: {count}");
                    }
                    if validate_message(&message) {
                        println!("Message found at position: {count}");
                    }
                }
            }
        }
    }
}

fn validate_marker(marker: &[char; 4]) -> bool
{
    // If it still contains a placeholder it is false
    if marker.contains(&'A') {
        return false;
    }
    let iter = marker.into_iter();
    let mut check: Vec<char> = Vec::new();
    for item in iter {
        if check.contains(&item) {
            return false;
        }
        check.push(*item);
    }
    true
}

fn validate_message(marker: &[char; 14]) -> bool
{
    // If it still contains a placeholder it is false
    if marker.contains(&'A') {
        return false;
    }
    let iter = marker.into_iter();
    let mut check: Vec<char> = Vec::new();
    for item in iter {
        if check.contains(&item) {
            return false;
        }
        check.push(*item);
    }
    true
}
