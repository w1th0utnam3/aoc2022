use core::panic;

fn task1() {
    let mut signal_strengths = Vec::new();

    let mut register = 1;
    let mut cycle = 1;

    let mut push_signal_strength = |cycle: i64, register: i64| {
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            signal_strengths.push((cycle, register, cycle * register));
        }
    };

    for instruction in INPUT.lines() {
        push_signal_strength(cycle, register);

        if let Some(add) = instruction.strip_prefix("addx ") {
            let add = add
                .parse::<i64>()
                .expect("addx has to be followed by a valid integer");
            cycle += 1;
            push_signal_strength(cycle, register);
            cycle += 1;
            register += add;
        } else if instruction == "noop" {
            cycle += 1;
            // Noop
        } else {
            panic!("unknown instruction");
        }
    }

    signal_strengths
        .iter()
        .for_each(|&(cycle, _, strength)| println!("{cycle}: {strength}"));
    let sum: i64 = signal_strengths
        .iter()
        .map(|&(_, _, strength)| strength)
        .sum();
    println!("sum: {sum}");
}

fn task2() {
    let mut register: i64 = 1;
    let mut cycle: usize = 1;

    let mut screen = String::new();

    let mut draw_pixel = |cycle: usize, register: i64| {
        let pos = ((cycle - 1) % 40) as i64;
        if (register - pos).abs() <= 1 {
            screen.push('X');
        } else {
            screen.push('.')
        }

        if pos == 39 {
            screen.push('\n');
        }
    };

    for instruction in INPUT.lines() {
        draw_pixel(cycle, register);

        if let Some(add) = instruction.strip_prefix("addx ") {
            let add = add
                .parse::<i64>()
                .expect("addx has to be followed by a valid integer");
            cycle += 1;
            draw_pixel(cycle, register);
            cycle += 1;
            register += add;
        } else if instruction == "noop" {
            cycle += 1;
            // Noop
        } else {
            panic!("unknown instruction");
        }
    }

    print!("{screen}");
}

fn main() {
    task2();
}

const INPUT: &'static str = "addx 2
addx 3
addx 3
addx -2
addx 4
noop
addx 1
addx 4
addx 1
noop
addx 4
addx 1
noop
addx 2
addx 5
addx -28
addx 30
noop
addx 5
addx 1
noop
addx -38
noop
noop
noop
noop
addx 5
addx 5
addx 3
addx 2
addx -2
addx 2
noop
noop
addx -2
addx 12
noop
addx 2
addx 3
noop
addx 2
addx -31
addx 32
addx 7
noop
addx -2
addx -37
addx 1
addx 5
addx 1
noop
addx 31
addx -25
addx -10
addx 13
noop
noop
addx 18
addx -11
addx 3
noop
noop
addx 1
addx 4
addx -32
addx 15
addx 24
addx -2
noop
addx -37
noop
noop
noop
addx 5
addx 5
addx 21
addx -20
noop
addx 6
addx 19
addx -5
addx -8
addx -22
addx 26
addx -22
addx 23
addx 2
noop
noop
noop
addx 8
addx -10
addx -27
addx 33
addx -27
noop
addx 34
addx -33
addx 2
addx 19
addx -12
addx 11
addx -20
addx 12
addx 18
addx -11
addx -14
addx 15
addx 2
noop
addx 3
addx 2
noop
noop
noop
addx -33
noop
addx 1
addx 2
noop
addx 3
addx 4
noop
addx 1
addx 2
noop
noop
addx 7
addx 1
noop
addx 4
addx -17
addx 18
addx 5
addx -1
addx 5
addx 1
noop
noop
noop
noop";
