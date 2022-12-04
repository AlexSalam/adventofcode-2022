use crate::shared::read_lines;

pub fn check_eclipses()
{
    if let Ok(lines) = read_lines("./src/data/4/data.txt") {
        let mut eclipses = 0;
        let mut overlaps = 0;
        for line in lines {
            if let Ok(item) = line {
                let ranges: Vec<&str> = item.split(",").collect();
                if ranges.len() == 2 {
                    let range_one: Vec<&str> = ranges[0].split("-").collect();
                    let range_two: Vec<&str> = ranges[1].split("-").collect();
                    // Check eclipses
                    if first_eclipses_second(&range_one, &range_two) || second_eclipses_first(&range_one, &range_two) {
                        eclipses = eclipses + 1;
                    }
                    // Check overlap
                    if overlap(&range_one, &range_two) {
                        overlaps = overlaps + 1;
                    }
                } else {
                    println!("Should always count 2");
                }
            }
        }
        println!("Total eclipses: {eclipses}");
        println!("Total overlaps: {overlaps}");
    }
}

fn first_eclipses_second(range_one: &Vec<&str>, range_two: &Vec<&str>) -> bool
{
    (range_one[0].parse::<i32>().unwrap() <= range_two[0].parse::<i32>().unwrap()) && (range_one[1].parse::<i32>().unwrap() >= range_two[1].parse::<i32>().unwrap())
}

fn second_eclipses_first(range_one: &Vec<&str>, range_two: &Vec<&str>) -> bool
{
    (range_two[0].parse::<i32>().unwrap() <= range_one[0].parse::<i32>().unwrap()) && (range_two[1].parse::<i32>().unwrap() >= range_one[1].parse::<i32>().unwrap())
}

fn overlap(range_one: &Vec<&str>, range_two: &Vec<&str>) -> bool
{
    (range_one[1].parse::<i32>().unwrap() >= range_two[0].parse::<i32>().unwrap()) && range_two[1].parse::<i32>().unwrap() >= range_one[0].parse::<i32>().unwrap()
}
