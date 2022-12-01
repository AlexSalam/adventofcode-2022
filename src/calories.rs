use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn count()
{
    if let Ok(lines) = read_lines("./src/data/1/data.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut max_calories = 0;
        let mut cals = 0;

        for line in lines {
            if let Ok(item) = line {
                if str::is_empty(&item) {
                    if cals > max_calories {
                        max_calories = cals;
                    }
                    cals = 0;
                    continue;
                } else {
                    cals = cals + item.parse::<i32>().unwrap();
                }
            }
        }

        println!("{max_calories}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
