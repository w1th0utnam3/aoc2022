static INPUT: &str = "A Y
B X
C Z";

const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';

const LOSE: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

fn shape_score(s: char) -> usize {
    match s {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => panic!("invalid shape"),
    }
}

fn win_score(other_hand: char, own_hand: char) -> usize {
    match (other_hand, own_hand) {
        (ROCK, PAPER) | (PAPER, SCISSORS) | (SCISSORS, ROCK) => 6,
        (ROCK, SCISSORS) | (PAPER, ROCK) | (SCISSORS, PAPER) => 0,
        _ if other_hand == own_hand => 3,
        _ => panic!("invalid play"),
    }
}

fn task1() {
    let decrypt = |encrypted: char| -> char {
        match encrypted {
            'X' => ROCK,
            'Y' => PAPER,
            'Z' => SCISSORS,
            _ => panic!("invalid shape"),
        }
    };

    let score = INPUT
        .lines()
        .into_iter()
        .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
        .map(|(a, b)| (a, decrypt(b)))
        .map(|(a, b)| shape_score(b) + win_score(a, b))
        .sum::<usize>();

    println!("score: {score}");
}

fn task2() {
    let decrypt = |other_hand: char, result: char| -> char {
        match result {
            LOSE => match other_hand {
                ROCK => SCISSORS,
                PAPER => ROCK,
                SCISSORS => PAPER,
                _ => panic!("invalid shape"),
            },
            DRAW => other_hand,
            WIN => match other_hand {
                ROCK => PAPER,
                PAPER => SCISSORS,
                SCISSORS => ROCK,
                _ => panic!("invalid shape"),
            },
            _ => panic!("invalid result"),
        }
    };

    let score = INPUT
        .lines()
        .into_iter()
        .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
        .map(|(a, b)| (a, decrypt(a, b)))
        .map(|(a, b)| shape_score(b) + win_score(a, b))
        .sum::<usize>();

    println!("score: {score}");
}

fn main() {
    task2();
}
