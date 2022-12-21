use std::collections::HashMap;

pub fn get_score(){
    let a = std::fs::read_to_string("/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day2/input.txt").unwrap();

    let  m_ix: HashMap<&str, usize> = HashMap::from([
        ("A",0),("B",1),("C",2),
        ("X",0),("Y",1),("Z",2)]);

    // let m_pts:[[i32;3];3] = [[4,8,3],
    //                         [1,5,9],
    //                         [7,2,6]];

    let m_pts:Vec<Vec<u32>> = vec![vec![3,4,8],
                            vec![1,5,9],
                            vec![2,6,7]];
    
    let b =
        a.lines().map(|s| s.split_whitespace().collect::<Vec<&str>>());

    let mut _sum = 0;

    for spl in b{
        _sum += m_pts[(*m_ix.get(spl[0]).unwrap())][(*m_ix.get(spl[1]).unwrap())];
    }
    println!("{}",_sum);    

} 