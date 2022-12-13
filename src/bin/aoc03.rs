use std::collections::HashSet;

fn get_priority(c: char) -> usize {
    assert!(
        c.is_ascii() && c.is_alphabetic(),
        "characters must be ascii alphabetic"
    );

    if c.is_lowercase() {
        ((c as u8) - ('a' as u8) + 1) as usize
    } else {
        ((c as u8) - ('A' as u8) + 27) as usize
    }
}

fn to_char_set(s: &str) -> HashSet<char> {
    s.chars().collect::<HashSet<_>>()
}

fn task1() {
    let sum = INPUT
        .lines()
        .into_iter()
        .filter(|l| l.len() > 0)
        .inspect(|l| assert!(l.len() % 2 == 0, "each line length must be divisible by 2"))
        .map(|l| l.split_at(l.len() / 2))
        .map(|(first, second)| (to_char_set(first), second))
        .map(|(first_set, second)| {
            second
                .chars()
                .find(|c| first_set.contains(&c))
                .expect("rucksack must contain at least one duplicate")
        })
        //.inspect(|res| println!("{res:?}"))
        .map(get_priority)
        //.inspect(|res| println!("{res:?}"))
        .sum::<usize>();

    println!("sum: {sum}");
}

fn task2() {
    let lines = INPUT.lines().map(Into::into).collect::<Vec<String>>();
    assert!(
        lines.len() % 3 == 0,
        "number of lins must be divisible by 3"
    );

    let sum = lines
        .chunks_exact(3)
        .map(|group| {
            (
                to_char_set(&group[0])
                    .intersection(&to_char_set(&group[1]))
                    .copied()
                    .collect::<HashSet<_>>(),
                &group[2],
            )
        })
        .map(|(first_set, last)| {
            last.chars()
                .find(|c| first_set.contains(&c))
                .expect("rucksack must contain at least one badge")
        })
        //.inspect(|res| println!("{res:?}"))
        .map(get_priority)
        //.inspect(|res| println!("{res:?}"))
        .sum::<usize>();

    println!("sum: {sum}");
}

fn main() {
    task2();
}

static INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
