use std::ops::Range;

fn parse_range(s: &str) -> Range<usize> {
    let (start, end) = s.split_once('-').expect("ranges should contain a dash");
    start.parse::<usize>().unwrap()..end.parse::<usize>().unwrap()
}

fn range_includes(first: &Range<usize>, second: &Range<usize>) -> bool {
    first.start <= second.start && first.end >= second.end
}

fn range_overlap(first: &Range<usize>, second: &Range<usize>) -> bool {
    second.start >= first.start && second.start <= first.end
        || second.end >= first.start && second.end <= first.end
}

fn task1() {
    let include_count = INPUT
        .lines()
        .into_iter()
        .filter(|l| l.len() > 0)
        .map(|l| l.split_once(',').expect("each line should contain comma"))
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        //.inspect(|res| println!("{res:?}"))
        .map(|(a, b)| range_includes(&a, &b) || range_includes(&b, &a))
        .map(|includes| includes as usize)
        //.inspect(|res| println!("{res:?}"))
        .sum::<usize>();

    println!("include_count: {include_count}");
}

fn task2() {
    let overlap_count = INPUT
        .lines()
        .into_iter()
        .filter(|l| l.len() > 0)
        .map(|l| l.split_once(',').expect("each line should contain comma"))
        .map(|(a, b)| (parse_range(a), parse_range(b)))
        //.inspect(|res| println!("{res:?}"))
        .map(|(a, b)| range_overlap(&a, &b) || range_overlap(&b, &a))
        .map(|includes| includes as usize)
        //.inspect(|res| println!("{res:?}"))
        .sum::<usize>();

    println!("overlap_count: {overlap_count}");
}

fn main() {
    task2();
}

static INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
