use std::env;

extern crate pathfinding;

// Shared modules
pub mod shared;

// Puzzle modules
pub mod calories;
pub mod rps;
pub mod backpacks;
pub mod ranges;
pub mod cargo;
pub mod transmissions;
pub mod filesystem;
pub mod treepatch;
pub mod rope;
pub mod cpu;
pub mod monkeys;
pub mod hills;

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
                1 => calories::count(),
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
                3 => backpacks::count_duplicate_priorities(),
                4 => ranges::check_eclipses(),
                5 => cargo::top_of_stack(),
                6 => transmissions::get_first_marker(),
                7 => filesystem::run(),
                8 => treepatch::visible(),
                9 => {
                    rope::visitations();
                    rope::chain_visitations();
                },
                10 => {
                    // cpu::sum_signals();
                    cpu::draw();
                },
                11 => monkeys::business(),
                12 => hills::route(),
                _ => println!("Values must be between 1 and 25"),
            }
        }
    }

}
