use crate::shared::read_lines;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Outcome {
    Win,
    Loss,
    Draw
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Selection {
    Rock,
    Paper,
    Scissors
}

pub enum ColumnOption {
    Outcome,
    Me
}

pub fn strategy(option: ColumnOption)
{
    let mut total = 0;

    // Opponent picks
    let mut selection_map: HashMap<char, Selection> = HashMap::new();
    selection_map.insert('A', Selection::Rock);
    selection_map.insert('B', Selection::Paper);
    selection_map.insert('C', Selection::Scissors);
    selection_map.insert('X', Selection::Rock);
    selection_map.insert('Y', Selection::Paper);
    selection_map.insert('Z', Selection::Scissors);

    let mut outcome_map: HashMap<char, Outcome> = HashMap::new();
    outcome_map.insert('X', Outcome::Loss);
    outcome_map.insert('Y', Outcome::Draw);
    outcome_map.insert('Z', Outcome::Win);
    // Scoring
    let mut scores: HashMap<Selection, i32> = HashMap::new();
    scores.insert(Selection::Rock, 1);
    scores.insert(Selection::Paper, 2);
    scores.insert(Selection::Scissors, 3);

    if let Ok(lines) = read_lines("./src/data/2/data.txt") {

        for line in lines {
            if let Ok(battle) = line {
                let char_vec: Vec<char> = battle.chars().collect();
                let opponent_selection = &selection_map[&char_vec[0]];
                let my_selection = match option {
                    ColumnOption::Outcome => {
                        &action(&opponent_selection, &outcome_map[&char_vec[2]])
                    }
                    ColumnOption::Me => {
                        &selection_map[&char_vec[2]]
                    }
                };
                match compare(&opponent_selection, &my_selection) {
                    Outcome::Win => {
                        total = total + 6;
                    },
                    Outcome::Loss => {
                        total = total + 0;
                    }
                    Outcome::Draw => {
                        total = total + 3;
                    }
                }
                total = total + scores[&my_selection];
            }
        }
        println!("Total score: {total}");
    }
}

// Compare and return the result
fn compare(opponent: &Selection, me: &Selection) -> Outcome
{
    match opponent {
        Selection::Rock => {
            match me {
                Selection::Rock => {
                    Outcome::Draw
                },
                Selection::Paper => {
                    Outcome::Win
                },
                Selection::Scissors => {
                    Outcome::Loss
                },
            }
        },
        Selection::Paper => {
            match me {
                Selection::Rock => {
                    Outcome::Loss
                },
                Selection::Paper => {
                    Outcome::Draw
                },
                Selection::Scissors => {
                    Outcome::Win
                }
            }
        },
        Selection::Scissors => {
            match me {
                Selection::Rock => {
                    Outcome::Win
                },
                Selection::Paper => {
                    Outcome::Loss
                },
                Selection::Scissors => {
                    Outcome::Draw
                }
            }
        }
    }
}

// Take the result and outcome then predict my action
fn action<'a>(opponent: &Selection, outcome: &Outcome) -> &'a Selection
{
    match opponent {
        Selection::Rock => {
            match outcome {
                Outcome::Draw => {
                    &Selection::Rock
                },
                Outcome::Win => {
                    &Selection::Paper
                },
                Outcome::Loss => {
                    &Selection::Scissors
                },
            }
        },
        Selection::Paper => {
            match outcome {
                Outcome::Draw => {
                    &Selection::Paper
                },
                Outcome::Win => {
                    &Selection::Scissors
                },
                Outcome::Loss => {
                    &Selection::Rock
                },
            }
        },
        Selection::Scissors => {
            match outcome {
                Outcome::Draw => {
                    &Selection::Scissors
                },
                Outcome::Win => {
                    &Selection::Rock
                },
                Outcome::Loss => {
                    &Selection::Paper
                },
            }
        }
    }
}
