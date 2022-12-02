use std::env;

// Shared modules
pub mod shared;

// Puzzle modules
pub mod calories;
pub mod rps;

// Main application
fn main() {
    // Get the descired challange day from the command line
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<i8>();

    // Validate that this is an acceptable integer
    match day {
        Err(_) => {
            println!("Must be a valid integer");
        }
        Ok(day) => {
            println!("Doing puzzle for day: {}", day);
            // Run the module method for this puzzle
            match day {
                1 => {
                    calories::count();
                }
                2 => {
                    if args.len() == 3 {
                        match args[2].as_str() {
                            "outcome" => rps::strategy(rps::ColumnOption::Outcome),
                            "me" => rps::strategy(rps::ColumnOption::Me),
                            _ => rps::strategy(rps::ColumnOption::Me)
                        }
                    } else {
                        rps::strategy(rps::ColumnOption::Me);
                    }

                }
                _ => {
                    println!("Values must be between 1 and 25");
                }
            }
        }
    }

}
