use std::collections::HashSet;

fn apply_step(mut pos: [i32; 2], dir: &str) -> [i32; 2] {
    match dir {
        "R" => pos[0] += 1,
        "L" => pos[0] -= 1,
        "U" => pos[1] += 1,
        "D" => pos[1] -= 1,
        _ => panic!("unknown direction"),
    }
    pos
}

fn step_towards(mut from: [i32; 2], towards: [i32; 2]) -> [i32; 2] {
    if (towards[0] - from[0]).abs() > 1 || (towards[1] - from[1]).abs() > 1 {
        from[0] += (towards[0] - from[0]).signum();
        from[1] += (towards[1] - from[1]).signum();
    }

    from
}

fn task1() {
    let mut pos_head = [0, 0];
    let mut pos_tail = [0, 0];

    let mut unique_positions = HashSet::new();

    for line in INPUT.lines() {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps = steps.parse::<i32>().unwrap();
        for _ in 0..steps {
            pos_head = apply_step(pos_head, dir);
            pos_tail = step_towards(pos_tail, pos_head);
            unique_positions.insert(pos_tail);
            println!("{pos_tail:?}");
        }
    }

    //println!("tail positions: {unique_positions:?}");
    println!("unique position count: {}", unique_positions.len());
}

fn task2() {
    let mut knot_positions: Vec<[i32; 2]> = vec![[0, 0]; 10];
    let mut unique_positions = HashSet::<[i32; 2]>::new();

    INPUT
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .flat_map(|(dir, steps)| {
            let steps = steps.parse::<usize>().unwrap();
            std::iter::repeat(dir).take(steps)
        })
        .for_each(|dir| {
            knot_positions[0] = apply_step(knot_positions[0], dir);

            for i in 0..(knot_positions.len() - 1) {
                if let [knot_front, knot_back] = &mut knot_positions[i..i + 2] {
                    *knot_back = step_towards(*knot_back, *knot_front);
                } else {
                    panic!();
                }
            }

            let last = knot_positions.last().unwrap().clone();
            unique_positions.insert(last);
            println!("{last:?}");
        });

    println!("unique position count: {}", unique_positions.len());
}

fn main() {
    task2();
}

const INPUT: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
