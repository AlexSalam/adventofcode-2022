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
    };
    let moves: Vec<(Direction, i8)> = read_moves();
    grid.calc_tail_visitations(moves);
}
