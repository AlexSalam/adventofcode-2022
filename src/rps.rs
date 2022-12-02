use crate::shared::read_lines;
use std::collections::HashMap;

enum BattleOutcome {
    Win,
    Loss,
    Draw,
    Unknown
}

pub fn strategy()
{
    // Out point Total
    let mut total = 0;

    // Store points for our choice
    let mut choice_points = HashMap::new();
    choice_points.insert('X', 1);
    choice_points.insert('Y', 2);
    choice_points.insert('Z', 3);


    if let Ok(lines) = read_lines("./src/data/2/data.txt") {
        for line in lines {
            if let Ok(battle) = line {
                let char_vec: Vec<char> = battle.chars().collect();
                match resolve_battle(char_vec[0], char_vec[2]) {
                    BattleOutcome::Win => {
                        total = total + 6;
                    },
                    BattleOutcome::Loss => {

                    },
                    BattleOutcome::Draw => {
                        total = total + 3
                    },
                    BattleOutcome::Unknown => {
                        continue;
                    }
                }
                // Append our choice score
                if let Some(choice_score) = choice_points.get(&char_vec[2]) {
                    total = total + choice_score;
                }
            }
        }
        println!("Total score: {total}");
    }
}

// ROCK: A, X
// Paper B, Y
// Scissors C, Z

// get the outcome of the duel
fn resolve_battle(them: char, me: char) -> BattleOutcome
{
    match me {
        'X' => {
            match them {
                'A' => {
                    return BattleOutcome::Draw
                },
                'B' => {
                    return BattleOutcome::Loss
                },
                'C' => {
                    return BattleOutcome::Win
                },
                _ => {
                    return BattleOutcome::Unknown
                }
            }
        },
        'Y' => {
            match them {
                'A' => {
                    return BattleOutcome::Win
                },
                'B' => {
                     return BattleOutcome::Draw
                },
                'C' => {
                    return BattleOutcome::Loss
                },
                _ => {
                    return BattleOutcome::Unknown
                }            }
        },
        'Z' => {
            match them {
                'A' => {
                     return BattleOutcome::Loss
                },
                'B' => {
                     return BattleOutcome::Win
                },
                'C' => {
                     return BattleOutcome::Draw
                 },
                 _ => {
                     return BattleOutcome::Unknown
                 }            }
        },
        _ => {
            return BattleOutcome::Unknown
        }
    }
}
