use std::collections::BTreeSet;

fn find_magic_bytes(length: usize) -> usize {
    let (window_index, _) = INPUT
        .as_bytes()
        .windows(length)
        .enumerate()
        .find(|(idx, w)| w.iter().collect::<BTreeSet<_>>().len() == length)
        .expect("no marker found");
    window_index
}

fn task1() {
    let chars_to_read = find_magic_bytes(4) + 4;
    println!("characters to read: {chars_to_read}");
}

fn task2() {
    let chars_to_read = find_magic_bytes(14) + 14;
    println!("characters to read: {chars_to_read}");
}

fn main() {
    task2();
}

static INPUT: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
