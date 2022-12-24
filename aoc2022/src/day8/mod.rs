use array2d::Array2D;
use ndarray::{prelude::*, OwnedRepr, RawData, ViewRepr};
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

pub fn compute_solution_2(input: String) {
    let n_rows = input.matches("\n").count() + 1;
    let chars = input.replace("\n", "");
    let n_columns = chars.len() / n_rows;
    let mut tree_heights = chars
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut trees = ArrayViewMut2::from_shape((n_rows, n_columns), &mut tree_heights).unwrap();

    let mut product_view_lengths: ArrayBase<OwnedRepr<u32>, Dim<[usize; 2]>> = Array2::ones((n_rows, n_columns));

    for i in 0..4 {
        // flip then transpose the source array i times
        trees = rotate_right(trees, i);

        // Get a mutable array for the i't view lengths
        let mut view_lengths = Array2::zeros((n_rows, n_columns));
        view_lengths[[0, 0]] = 1;

        // Compute the view lengths and store in view length array
        for ii in 0..n_rows {
            for jj in 0..n_columns {
                view_lengths[[ii, jj]] = compute_view_length(&trees, ii, jj);
            }
        }

        // transpose then flip the source and the computed array i times
        trees = rotate_left(trees, i);
        view_lengths = rotate_left(view_lengths, i);

        // Store the result in the vec
        product_view_lengths = product_view_lengths * view_lengths;
    }

    println!("{}", product_view_lengths.iter().max().unwrap());
}

fn compute_view_length<T>(arr: &ArrayViewMut2<T>, row: usize, col: usize) -> u32
where
    T: PartialOrd,
{
    // For the specified row, count the number of trees to the right before a tree at least as big is encountered
    for (view_length, tree_index) in (col + 1..arr.ncols()).enumerate() {
        if arr[[row, tree_index]] >= arr[[row, col]] {
            return (view_length + 1) as u32;
        }
    }
    (arr.ncols() - col - 1) as u32
}

fn rotate_right<T, D: ndarray::Dimension>(mut arr: ArrayBase<T, D>, count: u32) -> ArrayBase<T, D>
where
    T: RawData,
{
    for _ in 0..=count {
        arr = arr.reversed_axes();
        arr.invert_axis(Axis(0));
    }
    arr
}

fn rotate_left<T, D: ndarray::Dimension>(mut arr: ArrayBase<T, D>, count: u32) -> ArrayBase<T, D>
where
    T: RawData,
{
    for _ in 0..=count {
        arr.invert_axis(Axis(0));
        arr = arr.reversed_axes();
    }
    arr
}
