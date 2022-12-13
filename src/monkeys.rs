use crate::shared::read_lines;

#[derive(Clone, Debug)]
struct Game {
    monkeys: Vec<Monkey>,
    round: i32,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u128>,
    true_target: i32,
    false_target: i32,
    inspections: i32
}

struct Toss {
    item: u128,
    from: usize,
    to: usize,
    index: usize
}

impl Game {
    fn inspect(&mut self, monkey: usize, index: usize)
    {
        let item = self.monkeys[monkey].items[index];
        let result: u128 = match monkey {
            0 => item * 7,
            1 => item + 4,
            2 => item + 2,
            3 => item + 7,
            4 => item * 17,
            5 => item + 8,
            6 => item + 6,
            7 => item * item,
            _ => {
                println!("Inspection weirdness!");
                item
            }
        };
        // let result: i64 = match monkey {
        //     0 => item * 2,
        //     1 => item * item,
        //     2 => item + 6,
        //     3 => item + 2,
        //     4 => item * 11,
        //     5 => item + 7,
        //     6 => item + 1,
        //     7 => item + 5,
        //     _ => {
        //         println!("Inspection weirdness!");
        //         item
        //     }
        // };
        let result = result %  9699690;
        // println!("For monkey: {} item: {} is now: {}", monkey, item, result);
        self.monkeys[monkey].inspections = self.monkeys[monkey].inspections + 1;
        self.monkeys[monkey].items[index] = result;
    }

    fn test(&mut self, monkey: usize, index: usize) -> Toss
    {
        let item = self.monkeys[monkey].items[index];
        let result: bool = match monkey {
            0 => item % 17 == 0,
            1 => item % 3 == 0,
            2 => item % 5 == 0,
            3 => item % 7 == 0,
            4 => item % 11 == 0,
            5 => item % 19 == 0,
            6 => item % 2 == 0,
            7 => item % 13 == 0,
            _ => {
                println!("Test weirdness!");
                false
            }
        };
        // let result: bool = match monkey {
        //     0 => item % 11 == 0,
        //     1 => item % 19 == 0,
        //     2 => item % 7 == 0,
        //     3 => item % 17 == 0,
        //     4 => item % 3 == 0,
        //     5 => item % 5 == 0,
        //     6 => item % 13 == 0,
        //     7 => item % 2 == 0,
        //     _ => {
        //         println!("Test weirdness!");
        //         false
        //     }
        // };
        let mut target: usize = 0;
        if result {
            target = self.monkeys[monkey].true_target as usize;
        } else {
            target = self.monkeys[monkey].false_target as usize;
        }
        // println!("Monkey {} gains: {} from monkey {} - test was {}", target, item, monkey, result);

        Toss {
            item: item,
            to: target,
            from: monkey,
            index: index
        }
    }

    fn print_total_items(&self)
    {
        let mut count = 0;
        for monkey in self.monkeys.iter() {
            for item in monkey.items.iter() {
                count = count + 1;
            }
        }
        println!("Total items: {}", count);
    }

    fn print_inventories(&self)
    {
        let mut count = 0;
        for monkey in &self.monkeys {
            let item_string: String = monkey.items.iter().map( |&id| id.to_string() + ",").collect();
            println!("Monkey {} has {}", count, item_string);
            count = count + 1;
        }
    }

    pub fn game_until(&mut self, round: i32)
    {
        while self.round <= round {
            println!("Round is now: {}", self.round);
            // self.print_total_items();
            for monkey in 0..8 {
                self.take_turn(monkey);
            }
            self.round = self.round + 1;
            // self.print_total_items();
            // self.print_inventories();
        }
    }

    pub fn take_turn(&mut self, monkey: usize)
    {
        let items = self.monkeys[monkey].items.clone();
        if items.len() == 0 {
            return ();
        }
        let mut to_throw: Vec<Toss> = Vec::new();
        for (index, item) in items.iter().enumerate() {
            self.inspect(monkey, index);
            to_throw.push(self.test(monkey, index));
        }
        for toss in to_throw.iter() {
            // println!("Throwing {} which is item {} from monkey {} to monkey {}", toss.item, toss.index, toss.from, toss.to);
            self.monkeys[toss.to].items.push(toss.item);
        }
        if to_throw.len() != items.len() {
            println!("Weirdness!");
        }
        self.monkeys[monkey].items = Vec::new();
    }

    pub fn print_monkey_business(&self)
    {
        let mut top_inspectors: [i64; 8] = [0; 8];
        let mut count = 0;
        for monkey in self.monkeys.iter() {
            top_inspectors[count] = monkey.inspections as i64;
            count = count + 1;
        }
        dbg!(&top_inspectors);
        top_inspectors.sort();
        println!("Monkey business: {}", top_inspectors[6] * top_inspectors[7]);
    }
}

// Attempt 312192 too big
// Attempt 027224 too low
// Attempt 110888 too high

pub fn business()
{
    let mut game: Game = create_game_state();
    // dbg!(&game);
    game.game_until(10000);
    game.print_monkey_business();
    // dbg!(&game);
}

fn create_game_state() -> Game
{
    let mut game: Game = Game {
        monkeys: Vec::new(),
        round: 1,
    };
    let mut number: i32 = 0;
    let mut current_monkey: Monkey = Monkey {
        items: Vec::new(),
        true_target: 0,
        false_target: 0,
        inspections: 0
    };
    if let Ok(lines) = read_lines("./src/data/11/data.txt") {
        for (index, line) in lines.enumerate() {
            if let Ok(item) = line {
                if item.contains("Monkey") && !item.contains("If") {
                    if index != 0 {
                        game.monkeys.push(current_monkey.clone());
                        current_monkey = Monkey {
                            items: Vec::new(),
                            true_target: 0,
                            false_target: 0,
                            inspections: 0
                        };
                    }
                    // Append a completed monkey to the game

                }
                if item.contains("Starting items") {
                    let starting: Vec<&str> = item.split(":").collect();
                    let things: Vec<&str> = starting[1].split(",").collect();
                    for thing in things {
                        current_monkey.items.push(thing.trim().parse::<u128>().unwrap());
                    }
                }
                if item.contains("true") {
                    current_monkey.true_target = item.chars().last().unwrap().to_digit(10).unwrap() as i32;
                }
                if item.contains("false") {
                    current_monkey.false_target = item.chars().last().unwrap().to_digit(10).unwrap() as i32;
                }
            }
        }
        game.monkeys.push(current_monkey.clone());
    }
    game
}
