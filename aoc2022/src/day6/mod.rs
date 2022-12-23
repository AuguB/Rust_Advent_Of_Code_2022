pub fn compute_solution_1(_input: String) {
    detect_n_consecutive_unique_characters(4, _input);
}

fn detect_n_consecutive_unique_characters(n:usize,input:String){
    for i in n..input.len() {
        let mut sorts = input[i-n..i].chars().collect::<Vec<char>>();
        sorts.sort();
        sorts.dedup();
        if sorts.len() == n {
            println!("{i}");
            break
        }
    }
}

pub fn compute_solution_2(input: String) {
    detect_n_consecutive_unique_characters(14, input)
}
