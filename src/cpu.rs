use crate::shared::read_lines;
use std::collections::HashMap;
use std::process::exit;

#[derive(Debug, Copy, Clone)]
enum CommandType {
    AddX,
    Noop
}

#[derive(Debug, Copy, Clone)]

struct Command {
    kind: CommandType,
    execution_time: i8
}

#[derive(Debug, Clone)]
struct Cpu {
    x_register: i32, // Current register value
    processing: Option<(Command, i32)>, // Current command
    important_cycle_mod: i32, // Number of cycles to get signal strength
    cycle: i32, // Cycle counter
    important_signals: Vec<i32>, // Recorded signal outputs at specific times defined by the cycle mod
    screen: HashMap<(i32, i32), bool>
}

// 21480 is too high

impl Cpu {
    pub fn run_program(&mut self, commands: Vec<(Command, i32)>)
    {
        for command in commands {
            if self.processing.is_none() {
                // println!("Setting command: {:?}", command.0.kind);
                self.processing = Option::Some(command);
            }
            while self.processing.is_some() {
                self.tick_cycle();
            }
        }
    }

    pub fn draw(&self)
    {
        println!("");
        println!("");
        // dbg!(&self.screen.len());
        // exit(1);
        let mut output: [[char; 40]; 6] = [['.'; 40]; 6];
        let output_copy = output.clone();
        for (y, row) in output_copy.into_iter().enumerate() {
            for (x, _column) in row.into_iter().enumerate() {
                let pixel = self.screen.get(&(x as i32, y as i32)).unwrap();
                if *pixel {
                    output[y][x] = '#';
                }
            }
        }
        for line in output {
            let string_output: String = line.iter().collect();
            println!("{}", string_output);
        }
    }

    fn add_pixel(&mut self)
    {
        // println!("Adding pixel for cycle: {}", self.cycle);
        let x: i32 = self.cycle % self.important_cycle_mod;
        let y: i32 = self.cycle / self.important_cycle_mod;

        let mut lit: bool = false;

        if x > 39 || y > 6 {
            println!("Invalid coords detected! ({},{})", x, y);
            exit(1);
        }

        if x == self.x_register || x == self.x_register - 1 || x == self.x_register + 1 {
            lit = true;
        }

        // println!("Pixel is: ({},{}) and is {}!", x, y, lit);
        self.screen.insert((x, y), lit);
    }

    fn tick_cycle(&mut self)
    {
        // New cycle
        self.add_pixel();
        self.cycle = self.cycle + 1;
        // println!("New cycle: {}", self.cycle);
        if self.cycle % self.important_cycle_mod == 20 {
            // Record this signal strength
            // println!("Recording signal: {} * {} = {}", self.x_register, self.cycle, self.x_register * self.cycle);
            self.important_signals.push(self.x_register * self.cycle);
        }
        if self.processing.is_some() {
            let mut current_instruction = self.processing.unwrap().clone();
            current_instruction.0.execution_time = current_instruction.0.execution_time - 1;
            if current_instruction.0.execution_time == 0 {
                match current_instruction.0.kind {
                    CommandType::AddX => {
                        // println!("Adding {} to X register!", current_instruction.1);
                        self.apply_to_x_register(current_instruction.1);
                        self.processing = Option::None;
                    },
                    CommandType::Noop => {
                        // println!("Nooping!");
                        self.processing = Option::None;
                    }
                }

            } else {
                self.processing = Option::Some(current_instruction);
            }
        }
    }

    fn apply_to_x_register(&mut self, value: i32)
    {
        self.x_register = self.x_register + value;
        // println!("X register is: {}", self.x_register);
    }

    pub fn sum_important_signals(&self)
    {
        let mut sum = 0;
        for signal in self.important_signals.iter() {
            sum = sum + signal;
        }
        println!("The sum of all important signals is: {}", sum);
    }
}

pub fn sum_signals()
{
    let commands: Vec<(Command, i32)> = read_commands();
    let mut cpu: Cpu = Cpu {
        x_register: 1,
        processing: Option::None,
        important_cycle_mod: 40,
        cycle: 0,
        important_signals: Vec::new(),
        screen: HashMap::new(),
    };
    cpu.run_program(commands);
    cpu.sum_important_signals();
}

pub fn draw()
{
    let commands: Vec<(Command, i32)> = read_commands();
    let mut cpu: Cpu = Cpu {
        x_register: 1,
        processing: Option::None,
        important_cycle_mod: 40,
        cycle: 0,
        important_signals: Vec::new(),
        screen: HashMap::new(),
    };
    cpu.run_program(commands);
    // dbg!(&cpu.screen);
    cpu.draw();
}

fn read_commands() -> Vec<(Command, i32)>
{
    let mut commands: Vec<(Command, i32)> = Vec::new();

    if let Ok(lines) = read_lines("./src/data/10/data.txt") {
        for line in lines {
            if let Ok(command_string) = line {
                let parts: Vec<&str> = command_string.split(" ").collect();
                match parts[0] {
                    "addx" => {
                        let command: Command = Command {
                            kind: CommandType::AddX,
                            execution_time: 2
                        };
                        commands.push((command, parts[1].parse::<i32>().unwrap()));
                    },
                    "noop" => {
                        let command: Command = Command {
                            kind: CommandType::Noop,
                            execution_time: 1
                        };
                        commands.push((command, 0));
                    }
                    _ => {
                        println!("Invalid command found!");
                    }
                }
            }
        }
    }

    commands
}
