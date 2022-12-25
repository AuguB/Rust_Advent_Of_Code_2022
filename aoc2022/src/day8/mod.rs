use ndarray::{prelude::*, OwnedRepr, RawData};
pub fn compute_solution_1(input: String) {
    let n_rows = input.matches("\n").count() + 1;
    let chars = input.replace("\n", "");
    let n_columns = chars.len() / n_rows;
    let mut tree_heights = chars
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut trees = ArrayViewMut2::from_shape((n_rows, n_columns), &mut tree_heights).unwrap();

    let mut seen_trees = Array2::from_elem((n_rows, n_columns),false);

    // Initiate a zero counter
    let mut number_of_seen_trees = 0;

    // // In all four directions, do the following:
    for rot in 0..4{
        //  For each line of vision:
        for row_i in 0..n_rows {
            //set a max_tree_height_counter to 0
            let mut max_tree_height: i32 = -1;
            //For each tree in the line of vision:
            for col_i in 0..n_columns{
                if trees[[row_i, col_i]] > max_tree_height {
                    // If the boolean value is false:
                    if !seen_trees[[row_i, col_i]] {
                        // set it to True
                        seen_trees[[row_i, col_i]] = true;
                        // Increment the count
                        number_of_seen_trees += 1;
                    }
                    // update the max_height
                    max_tree_height = trees[[row_i, col_i]];
                }
            }
        }
        trees = rotate_right(trees, 1);
        seen_trees = rotate_right(seen_trees, 1);
    }
    println!("{number_of_seen_trees}");
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
        // flip then transpose the source array 1 time
        trees = rotate_right(trees, 1);
        product_view_lengths = rotate_right(product_view_lengths, 1);

        // Get a mutable array for the i't view lengths
        let mut view_lengths = Array2::zeros((n_rows, n_columns));
        view_lengths[[0, 0]] = 1;

        // Compute the view lengths and store in view length array
        for ii in 0..n_rows {
            for jj in 0..n_columns {
                view_lengths[[ii, jj]] = compute_view_length(&trees, ii, jj);
            }
        }

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
    for _ in 0..count {
        arr = arr.reversed_axes();
        arr.invert_axis(Axis(0));
    }
    arr
}

