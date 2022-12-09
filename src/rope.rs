use crate::shared::read_lines;
use std::process::exit;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Err
}

struct Grid {
    head: (i32, i32),
    tail: (i32, i32),
    tail_visitations: Vec<(i32, i32)>,
    knots: [(i32, i32); 10]
}

impl Grid {

    pub fn calc_tail_visitations(&mut self, moves: Vec<(Direction, i8)>)
    {
        println!("Starting head: ({},{}) and tail: ({},{})", self.head.0, self.head.1, self.tail.0, self.head.0);
        for instruction in moves.iter() {
            println!("Doing {:?} {}", instruction.0, instruction.1);
            self.do_move(instruction.0, instruction.1);
        }
        println!("Total unique tail visitations: {}", self.tail_visitations.len());
    }

    pub fn calc_chain_movements(&mut self, moves: Vec<(Direction, i8)>)
    {
        println!("Starting head: ({},{}) and tail: ({},{})", self.knots[0].0, self.knots[0].1, self.knots[9].0, self.knots[9].1);
        let mut count = 0;
        for instruction in moves.iter() {
            println!("Doing {:?} {}", instruction.0, instruction.1);
            self.do_chain_move(instruction.0, instruction.1);
            // count = count + 1;
            // if count > 5 {
            //     exit(0);
            // }
        }
        println!("Total unique tail visitations: {}", self.tail_visitations.len());
    }

    // Attempt 11342 too high
    // Attempt 11274 too high
    fn do_chain_move(&mut self, direction: Direction, distance: i8)
    {
        let mut count = 0;
        while count < distance {
            let mut knots: [(i32, i32); 10] = self.knots.clone();
            match direction {
                Direction::Up => {
                    self.knots[0].1 = self.knots[0].1 + 1;
                }
                Direction::Down => {
                    self.knots[0].1 = self.knots[0].1 - 1;
                }
                Direction::Left => {
                    self.knots[0].0 = self.knots[0].0 - 1;
                }
                Direction::Right => {
                    self.knots[0].0 = self.knots[0].0 + 1;
                }
                Direction::Err => {
                    println!("How did you manage this?");
                }
            };
            println!("Knot 0 moved to: ({},{})", self.knots[0].0, self.knots[0].1);
            let mut leading_knot = self.knots[0].clone();
            for (index, knot) in knots.iter().enumerate() {
                if index == 0 {
                    continue;
                }
                // println!("For knot {}, leading_knot: ({},{}), knot: ({},{}), leading_knot_prev: ({},{})", index, leading_knot.0, leading_knot.1, knot.0, knot.1, leading_knot_prev.0, leading_knot_prev.1);
                self.knots[index] = self.propagate_movement(&leading_knot, &knot);
                leading_knot = self.knots[index].clone();
                println!("Knot {} moved to: ({},{})", index, self.knots[index].0, self.knots[index].1);
            }
            self.record_unique_chain_tail_position();
            count = count + 1;
        }
    }

    fn propagate_movement(&self, leading_knot: &(i32, i32), knot: &(i32, i32)) -> (i32, i32)
    {
        let mut new_knot = *knot;
        // If we are still adjacent do nothing
        if (knot.0 - leading_knot.0).abs() < 2 && (knot.1 - leading_knot.1).abs() < 2 {
            println!("Doing nothing!");
            return new_knot;
        }

        // Tail is too far away, we need to catch up
        // We have shifted by some x value
        if knot.1 == leading_knot.1 {
            println!("Shifting X!");
            new_knot.0 = (knot.0 + leading_knot.0) / 2;
            return new_knot;
        }
        // We have shifted by some y value
        if knot.0 == leading_knot.0 {
            println!("Shifting Y!");
            new_knot.1 = (knot.1 + leading_knot.1) / 2;
            return new_knot;
        }
        // We need to move diagonally 1 space to get as close as possible
        println!("Moving diagonally!");
        if new_knot.0 < leading_knot.0 {
            new_knot.0 = new_knot.0 + 1;
        } else {
            new_knot.0 = new_knot.0 - 1;
        }
        if new_knot.1 < leading_knot.1 {
            new_knot.1 = new_knot.1 + 1;
        } else {
            new_knot.1 = new_knot.1 - 1;
        }
        new_knot
    }

    fn record_unique_chain_tail_position(&mut self)
    {
        if !self.tail_visitations.contains(&self.knots[9]) {
            println!("Recording unique tail position: ({}, {})", self.knots[9].0, self.knots[9].1);
            self.tail_visitations.push(self.knots[9].clone());
        } else {
            println!("Weve been to ({},{}) before!", self.knots[9].0, self.knots[9].1);
        }
        ()
    }

    fn do_move(&mut self, direction: Direction, distance: i8)
    {
        let mut count = 0;
        while count < distance {
            let head = self.head.clone();
            match direction {
                Direction::Up => {
                    self.head.1 = self.head.1 + 1;
                }
                Direction::Down => {
                    self.head.1 = self.head.1 - 1;
                }
                Direction::Left => {
                    self.head.0 = self.head.0 - 1;
                }
                Direction::Right => {
                    self.head.0 = self.head.0 + 1;
                }
                Direction::Err => {
                    println!("How did you manage this?");
                }
            };
            self.catch_up_tail(head);
            println!("New positions for head: ({},{}) and tail: ({},{})", self.head.0, self.head.1, self.tail.0, self.tail.1);
            self.current_configuration_is_valid();
            self.record_unique_tail_position();
            count = count + 1;
        }
    }

    fn catch_up_tail(&mut self, prev_head: (i32, i32))
    {
        // If we are still adjacent do nothing
        if (self.tail.0 - self.head.0).abs() < 2 && (self.tail.1 - self.head.1).abs() < 2 {
            // println!("Doing nothing!");
            return ();
        }

        // Tail is too far away, we need to catch up
        // We have shifted by some x value
        if self.tail.1 == self.head.1 {
            // println!("Shifting X!");
            self.tail.0 = (self.tail.0 + self.head.0) / 2;
            return ();
        }
        // We have shifted by some y value
        if self.tail.0 == self.head.0 {
            // println!("Shifting Y!");
            self.tail.1 = (self.tail.1 + self.head.1) / 2;
            return ();
        }
        // We need to move diagonally to where the head was
        // println!("Moving a diagonal!");
        self.tail = prev_head;
        return ();
    }

    fn current_configuration_is_valid(&self)
    {
        if (self.tail.0 - self.head.0).abs() < 2 && (self.tail.1 - self.head.1).abs() < 2 {
            ()
        } else {
            println!("This configuration is invalid!");
            exit(1);
        }
    }

    fn record_unique_tail_position(&mut self)
    {
        if !self.tail_visitations.contains(&self.tail) {
            println!("Recording unique tail position: ({}, {})", self.tail.0, self.tail.1);
            self.tail_visitations.push(self.tail.clone());
        } else {
            println!("Weve been to ({},{}) before!", self.tail.0, self.tail.1);
        }
        ()
    }
}

pub fn read_moves() -> Vec<(Direction, i8)>
{
    let mut moves: Vec<(Direction, i8)> = Vec::new();
    if let Ok(lines) = read_lines("./src/data/9/data.txt") {
        for line in lines {
            if let Ok(instruction) = line {
                let move_parts: Vec<&str> = instruction.split(" ").collect();
                let direction: Direction = match move_parts[0] {
                    "D" => Direction::Down,
                    "U" => Direction::Up,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => Direction::Err
                };
                moves.push((direction, move_parts[1].parse::<i8>().unwrap()));
            }
        }
    }
    moves
}

// Attempt1 1834: too low

pub fn visitations()
{
    let mut grid: Grid = Grid {
        head: (0, 0),
        tail: (0, 0),
        tail_visitations: vec![(0, 0)],
        knots: [(0, 0); 10]
    };
    let moves: Vec<(Direction, i8)> = read_moves();
    grid.calc_tail_visitations(moves);
    // dbg!(grid.tail_visitations);
}

pub fn chain_visitations()
{
    let mut grid: Grid = Grid {
        head: (0, 0),
        tail: (0, 0),
        tail_visitations: vec![(0, 0)],
        knots: [(0, 0); 10]
    };
    let moves: Vec<(Direction, i8)> = read_moves();
    grid.calc_chain_movements(moves);
}
