use crate::shared::read_lines;

pub fn count()
{
    if let Ok(lines) = read_lines("./src/data/1/data.txt") {

        let mut max_calories = vec![0, 0, 0];
        let mut cals = 0;

        for line in lines {
            if let Ok(item) = line {
                if str::is_empty(&item) {
                    // Ensure the array is always in order
                    max_calories.sort();
                    // If the cals is bigger than the smallest value after sorting
                    if cals > max_calories[0] {
                        max_calories.remove(0);
                        max_calories.push(cals);
                    }
                    cals = 0;
                } else {
                    cals = cals + item.parse::<i32>().unwrap();
                }
            }
        }

        let mut total = 0;
        for item in max_calories {
            total = total + item;
        }
        println!("Total cals over top 3 elves: {total}");
    }
}
