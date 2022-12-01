use std::env;
use std::process::exit;

// Puzzle modules
pub mod calories;

// Main application
fn main() {
    // Get the descired challange day from the command line
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<i32>();

    // Validate that this is an acceptable integer
    match day {
        Err(_) => {
            println!("Must be a valid integer");
            exit(1);
        }
        Ok(day) => {
            println!("Doing puzzle for day: {}", day);
            // Run the module method for this puzzle
            match day {
                1 => {
                    calories::count();
                }
                _ => {
                    println!("Values must be between 1 and 25");
                }
            }
        }
    }

}
