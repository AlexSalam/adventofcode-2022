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
                    rps::strategy();
                }
                _ => {
                    println!("Values must be between 1 and 25");
                }
            }
        }
    }

}
