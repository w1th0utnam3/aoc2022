use regex::Regex;

fn initialize_stacks(start_config: &str) -> Vec<Vec<char>> {
    let num_stacks = start_config
        .lines()
        .last()
        .expect("start config needs at least one line")
        .trim_end()
        .rsplit_once(" ")
        .expect("wrong format of stack index line (missing withespace)")
        .1
        .parse::<usize>()
        .expect("could not parse stack index");

    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    // Fill the start configuration of the stacks
    {
        let re = Regex::new(r"[ \[](.)[ \]] ?").unwrap();

        if let Some((stack_start_config, _)) = start_config.rsplit_once("\n") {
            let stack_lines = stack_start_config.lines().collect::<Vec<_>>();
            for line in stack_lines.into_iter().rev() {
                assert!(
                    line.len() == num_stacks * 4 - 1,
                    "line does not have enough characters"
                );
                for (bx, stack) in re.captures_iter(line).zip(stacks.iter_mut()) {
                    let bx = bx[1].chars().next().expect("wrong box format");
                    if !bx.is_whitespace() {
                        stack.push(bx);
                    }
                }
            }
        } else {
            println!("Empty start configuration! Nothing to do!");
        }
    }

    stacks
}

fn task1() {
    let (start_config, instructions) = INPUT
        .split_once("\n\n")
        .expect("expected start config and instruction block separated by blank line");

    let mut stacks = initialize_stacks(start_config);

    // Execute instructions
    {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for line in instructions.lines() {
            let cap = re.captures(line).expect("invalid instruction format");
            let count = cap[1].parse::<usize>().expect("invalid instruction format");
            let from = cap[2].parse::<usize>().expect("invalid instruction format");
            let to = cap[3].parse::<usize>().expect("invalid instruction format");

            for _ in 0..count {
                let bx = stacks[from - 1].pop().expect("invalid instruction");
                stacks[to - 1].push(bx);
            }
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    print!("\n");
}

fn task2() {
    let (start_config, instructions) = INPUT
        .split_once("\n\n")
        .expect("expected start config and instruction block separated by blank line");

    let mut stacks = initialize_stacks(start_config);

    // Execute instructions
    {
        let mut buffer = Vec::new();
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for line in instructions.lines() {
            let cap = re.captures(line).expect("invalid instruction format");
            let count = cap[1].parse::<usize>().expect("invalid instruction format");
            let from = cap[2].parse::<usize>().expect("invalid instruction format");
            let to = cap[3].parse::<usize>().expect("invalid instruction format");

            for _ in 0..count {
                let bx = stacks[from - 1].pop().expect("invalid instruction");
                buffer.push(bx);
            }
            stacks[to - 1].extend(buffer.iter().rev());
            buffer.clear();
        }
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap_or(&' '));
    }
    print!("\n");
}

fn main() {
    task2();
}

static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
