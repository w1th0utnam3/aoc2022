fn task1() {
    let num_cols = INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .next()
        .expect("there has to be at least one non-empty line")
        .len();
    let mut num_rows = 0;

    let trees = INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .inspect(|_| num_rows += 1)
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("each character has to be a digit"))
        })
        .collect::<Vec<_>>();

    assert_eq!(trees.len(), num_cols * num_rows);

    let mut visible_trees = vec![false; trees.len()];

    fn check_visibility(
        outer: impl Iterator<Item = usize>,
        inner: impl Iterator<Item = usize> + Clone,
        num_cols: usize,
        trees: &[u32],
        visibility: &mut [bool],
        transpose: bool,
    ) {
        for i in outer {
            let mut max_tree = None;
            for j in inner.clone() {
                let flat_index = if !transpose {
                    i * num_cols + j
                } else {
                    j * num_cols + i
                };

                let tree = trees[flat_index];
                if max_tree.map(|max_tree| tree > max_tree).unwrap_or(true) {
                    max_tree = Some(tree);
                    visibility[flat_index] = true;
                }
            }
        }
    }

    check_visibility(
        0..num_rows,
        0..num_cols,
        num_cols,
        &trees,
        &mut visible_trees,
        false,
    );
    check_visibility(
        0..num_rows,
        (0..num_cols).into_iter().rev(),
        num_cols,
        &trees,
        &mut visible_trees,
        false,
    );
    check_visibility(
        0..num_cols,
        0..num_rows,
        num_cols,
        &trees,
        &mut visible_trees,
        true,
    );
    check_visibility(
        0..num_cols,
        (0..num_rows).into_iter().rev(),
        num_cols,
        &trees,
        &mut visible_trees,
        true,
    );

    let mut visible_count = 0;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if visible_trees[i * num_cols + j] {
                print!("X");
                visible_count += 1;
            } else {
                print!("O");
            }
        }
        print!("\n");
    }

    println!("visible trees: {visible_count}");
}

fn task2() {
    let num_cols = INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .next()
        .expect("there has to be at least one non-empty line")
        .len();
    let mut num_rows = 0;

    let trees = INPUT
        .lines()
        .filter(|s| !s.is_empty())
        .inspect(|_| num_rows += 1)
        .flat_map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("each character has to be a digit"))
        })
        .collect::<Vec<_>>();

    assert_eq!(trees.len(), num_cols * num_rows);

    let mut scenic_factor = vec![0; trees.len()];

    for i in 0..num_rows {
        for j in 0..num_cols {
            let candidate = trees[i * num_cols + j];

            // Right
            let mut right = 0;
            for k in (j + 1)..num_cols {
                let tree = trees[i * num_cols + k];
                right += 1;
                println!("{i} {j} sees {tree} (right)");
                if tree >= candidate {
                    break;
                }
            }

            // Left
            let mut left = 0;
            for k in (0..j).into_iter().rev() {
                let tree = trees[i * num_cols + k];
                left += 1;
                println!("{i} {j} sees {tree} (left)");
                if tree >= candidate {
                    break;
                }
            }

            // Bottom
            let mut bottom = 0;
            for k in (i + 1)..num_rows {
                let tree = trees[k * num_cols + j];
                bottom += 1;
                println!("{i} {j} sees {tree} (bottom)");
                if tree >= candidate {
                    break;
                }
            }

            // Top
            let mut top = 0;
            for k in (0..i).into_iter().rev() {
                let tree = trees[k * num_cols + j];
                top += 1;
                println!("{i} {j} sees {tree} (top)");
                if tree >= candidate {
                    break;
                }
            }

            scenic_factor[i * num_cols + j] = right * left * bottom * top;
        }
    }

    println!(
        "max scenic factor: {}",
        scenic_factor.iter().copied().max().unwrap_or(0)
    );
}

fn main() {
    task2();
}

const INPUT: &'static str = "30373
25512
65332
33549
35390";
