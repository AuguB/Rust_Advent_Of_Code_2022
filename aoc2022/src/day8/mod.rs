use array2d::Array2D;
pub fn compute_solution_1(input: String) {
    let n_rows = input.lines().count();
    let n_cols = input.lines().nth(0).unwrap().len();

    let input_ints = input
        .lines()
        .map(|l| l.chars().map(|c| (c.to_digit(10).unwrap() as isize)))
        .flatten();

    // input_ints.for_each(|f| println!("{f}"));
    let tree_grid = Array2D::from_iter_row_major(input_ints, n_rows, n_cols).unwrap();

    // Create a boolean array as big as the input
    // Start with all values to false
    let mut seen_grid = Array2D::filled_with(false, n_rows, n_cols);

    // Initiate a zero counter
    let mut number_of_seen_trees = 0;

    // In all four directions, do the following:
    //  For each line of vision:
    for row_i in 0..n_rows {
        //set a max_tree_height_counter to 0
        let mut max_tree_height: &isize = &-1;
        //For each tree in the line of vision:
        for (col_i, tree) in tree_grid.row_iter(row_i).unwrap().enumerate() {
            if *tree as isize > *max_tree_height {
                // If the boolean value is false:
                if !seen_grid.get(row_i, col_i).unwrap() {
                    // set it to True
                    seen_grid.set(row_i, col_i, true).unwrap();
                    // Increment the count
                    number_of_seen_trees += 1;
                }
                // update the max_height
                max_tree_height = &tree;
            }
        }
    }

    for row_i in 0..n_rows {
        let mut max_tree_height = &-1;
        for (col_i, tree) in tree_grid.row_iter(row_i).unwrap().rev().enumerate() {
            if tree > &max_tree_height {
                if !seen_grid.get(row_i, n_cols - col_i - 1).unwrap() {
                    seen_grid.set(row_i, n_cols - col_i - 1, true).unwrap();
                    number_of_seen_trees += 1;
                }
                max_tree_height = tree;
            }
        }
    }

    for col_i in 0..n_cols {
        let mut max_tree_height = &-1;
        for (row_i, tree) in tree_grid.column_iter(col_i).unwrap().enumerate() {
            if tree > &max_tree_height {
                if !seen_grid.get(row_i, col_i).unwrap() {
                    seen_grid.set(row_i, col_i, true).unwrap();
                    number_of_seen_trees += 1;
                }
                max_tree_height = tree;
            }
        }
    }

    for col_i in 0..n_cols {
        let mut max_tree_height = &-1;
        for (row_i, tree) in tree_grid.column_iter(col_i).unwrap().rev().enumerate() {
            if tree > &max_tree_height {
                if !seen_grid.get(n_rows - row_i - 1, col_i).unwrap() {
                    seen_grid.set(n_rows - row_i - 1, col_i, true).unwrap();
                    number_of_seen_trees += 1;
                }
                max_tree_height = tree;
            }
        }
    }
    println!("{number_of_seen_trees}");
}
fn print_visible(seen: &Array2D<bool>) {
    let rows = seen.as_rows();
    for row in rows.iter() {
        for col in row.iter() {
            let int = *col as u8;
            print!("{int}");
        }
        println!();
    }
    println!()
}

fn print_heights(seen: &Array2D<u32>) {
    let rows = seen.as_rows();
    for row in rows.iter() {
        for col in row.iter() {
            print!("{col}");
        }
        println!();
    }
    println!()
}

pub fn compute_solution_2(input: String) {}
