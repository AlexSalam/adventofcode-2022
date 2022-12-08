use crate::shared::read_lines;
use std::collections::HashMap;

// North/South are inverted from the visual input!
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn increment_x(&mut self)
    {
        self.x = self.x + 1;
    }

    pub fn increment_y(&mut self)
    {
        self.y = self.y + 1;
    }

    pub fn decrement_x(& mut self)
    {
        self.x = self.x - 1;
    }

    pub fn decrement_y(&mut self)
    {
        self.y = self.y - 1;
    }
}

struct Forest {
    trees: HashMap<Point, i8>,
    current: Point,
    visible_trees: i32,
}

impl Forest {
    pub fn count_visible_trees(&mut self)
    {
        self.visible_trees = 0;
        // Top left
        self.current = Point {
            x: 0,
            y: 0
        };

        loop {
            if self.current_tree_is_visible() {
                self.visible_trees = self.visible_trees + 1;
            }
            // Move one step right and check if we hit a boundary
            if self.go_right() {
                continue;
            } else {
                // We hit a boundary
                if self.next_row() {
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    pub fn count_maximum_scenic_score(&mut self)
    {
        self.visible_trees = 0;
        // Top left
        self.current = Point {
            x: 0,
            y: 0
        };

        let mut score = 0;
        loop {
            let this_score = self.current_tree_scenic_score();
            if this_score > score {
                score = this_score;
            }
            if self.go_right() {
                continue;
            } else {
                // We hit a boundary
                if self.next_row() {
                    continue;
                } else {
                    break;
                }
            }
        }
        println!("Max scenic score: {score}");
    }

    fn go_right(&mut self) -> bool
    {
        let mut peek_point = self.current;
        peek_point.increment_x();
        match self.trees.get(&peek_point) {
            Some(peek_tree) => {
                self.current = peek_point;
                return true;
            },
            None => {
                return false;
            }
        }
    }

    fn next_row(&mut self) -> bool
    {
        let mut peek_point = self.current;
        peek_point.increment_y();
        peek_point.x = 0;
        match self.trees.get(&peek_point) {
            Some(peek_tree) => {
                self.current = peek_point;
                return true;
            },
            None => {
                return false;
            }
        }
    }

    fn get_current_tree(&self) -> i8
    {
        *self.trees.get(&self.current).unwrap()
    }

    fn current_tree_scenic_score(&self) -> i32
    {
        self.check_scenic_score_in_direction(Direction::North) * self.check_scenic_score_in_direction(Direction::East) * self.check_scenic_score_in_direction(Direction::South) * self.check_scenic_score_in_direction(Direction::West)
    }

    fn current_tree_is_visible(&self) -> bool
    {
        self.check_visibility_in_direction(Direction::North) || self.check_visibility_in_direction(Direction::East) || self.check_visibility_in_direction(Direction::South) || self.check_visibility_in_direction(Direction::West)
    }

    fn check_scenic_score_in_direction(&self, direction: Direction) -> i32
    {
        let mut peek_point = self.current;
        let tree = self.get_current_tree();
        let mut distance = 0;
        println!("");
        println!("From tree of height: {} point ({},{})", tree, self.current.x, self.current.y);
        match direction {
            // Y + 1
            Direction::North => {
                println!("Looking north!");
                loop {
                    peek_point.increment_y();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            distance = distance + 1;
                            if peek_tree >= &tree {
                                return distance;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return distance;
            },
            // X + 1
            Direction::East => {
                println!("Looking east!");
                loop {
                    peek_point.increment_x();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            distance = distance + 1;
                            if peek_tree >= &tree {
                                return distance;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return distance;
            },
            // Y - 1
            Direction::South => {
                println!("Looking south!");
                loop {
                    if peek_point.y == 0 {
                        break;
                    }
                    peek_point.decrement_y();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            distance = distance + 1;
                            if peek_tree >= &tree {
                                return distance;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return distance;
            },
            // X - 1
            Direction::West => {
                println!("Looking west!");
                loop {
                    if peek_point.x == 0 {
                        break;
                    }
                    peek_point.decrement_x();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            distance = distance + 1;
                            if peek_tree >= &tree {
                                return distance;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return distance;
            }
        };
    }

    fn check_visibility_in_direction(&self, direction: Direction) -> bool
    {
        let mut peek_point = self.current;
        let mut at_edge = false;
        let tree = self.get_current_tree();
        println!("");
        println!("From tree of height: {} point ({},{})", tree, self.current.x, self.current.y);
        match direction {
            // Y + 1
            Direction::North => {
                println!("Looking north!");
                loop {
                    peek_point.increment_y();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            if peek_tree >= &tree {
                                return false;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return true;
            },
            // X + 1
            Direction::East => {
                println!("Looking east!");
                loop {
                    peek_point.increment_x();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            if peek_tree >= &tree {
                                return false;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return true;
            },
            // Y - 1
            Direction::South => {
                println!("Looking south!");
                loop {
                    if peek_point.y == 0 {
                        break;
                    }
                    peek_point.decrement_y();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            if peek_tree >= &tree {
                                return false;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return true;
            },
            // X - 1
            Direction::West => {
                println!("Looking west!");
                loop {
                    if peek_point.x == 0 {
                        break;
                    }
                    peek_point.decrement_x();
                    println!("Checking point ({},{})", peek_point.x, peek_point.y);
                    match self.trees.get(&peek_point) {
                        Some(peek_tree) => {
                            println!("Tree of height: {} found at ({},{})", peek_tree, peek_point.x, peek_point.y);
                            if peek_tree >= &tree {
                                return false;
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }
                return true;
            }
        };
    }
}

pub fn visible()
{
    let mut forest = Forest {
        trees: get_trees(),
        visible_trees: 0,
        current: Point {
            x: 0,
            y: 0
        }
    };
    forest.count_visible_trees();
    println!("There are {} visible trees.", forest.visible_trees);
    forest.count_maximum_scenic_score();
}

fn get_trees() -> HashMap<Point, i8>
{
    println!("Getting Trees!");
    let mut trees: HashMap<Point, i8> = HashMap::new();

    if let Ok(lines) = read_lines("./src/data/8/data.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(row) = line {
                let row_trees: Vec<char> = row.chars().collect();
                for (x, tree) in row_trees.iter().enumerate() {
                    trees.insert(Point {x: x, y: y}, tree.to_digit(10).unwrap() as i8);
                }
            }
        }
    }
    trees
}
