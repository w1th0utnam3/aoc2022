use std::error::Error;

static INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

fn main() -> Result<(), Box<dyn Error>> {
    let mut sums = INPUT
        .split("\n\n")
        .map(str::lines)
        .map(IntoIterator::into_iter)
        .map(|i| i.map(str::parse::<i64>).flatten().sum::<i64>())
        .collect::<Vec<_>>();

    sums.sort_unstable();

    let largest_three = sums.iter().rev().take(3).copied().collect::<Vec<_>>();
    let sum_largest_three = largest_three.iter().copied().sum::<i64>();

    println!("{sum_largest_three:?}");

    Ok(())
}
