use crate::shared::read_lines;

pub fn top_of_stack()
{
    if let Ok(lines) = read_lines("./src/data/5/data.txt") {

        let mut state = get_initial_state();
        for line in lines.skip(10) {
            if let Ok(instruction) = line {
                let halves = instruction.split(" from ").collect::<Vec<&str>>();
                let quantity = halves[0].split(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
                let from = halves[1].split(" to ").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
                let to = halves[1].split(" to ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
                state = move_crate(state, quantity, from, to);
            }
        }
        print_top_crates(state);
    }
}

fn print_top_crates(state: [Vec<char>; 9])
{
    let mut output: Vec<char> = vec![];
    for column in state.iter() {
        output.push(*column.last().unwrap());
    }
    let string_output: String = String::from(output.into_iter().collect::<String>());
    println!("Top crates: {string_output}");
}

fn move_crate(mut state: [Vec<char>; 9], quantity: usize, from: usize, to: usize) -> [Vec<char>; 9]
{
    let mut i = 0;
    while i < quantity {
        let cargo = state[from - 1].pop();
        match cargo {
            Some(item) => {
                state[to - 1].push(item);
            },
            None => println!("Something went amiss!")
        }
        i = i + 1;
    }
    state
}


fn get_initial_state() -> [Vec<char>; 9]
{
    [
        vec!['D', 'B', 'J', 'V'],
        vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'],
        vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'],
        vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'],
        vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'],
        vec!['R', 'D', 'B', 'S', 'N', 'G'],
        vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'],
        vec!['W', 'L', 'F'],
        vec!['S', 'V', 'F', 'M', 'R'],
    ]
}
