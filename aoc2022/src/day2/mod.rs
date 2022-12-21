use std::collections::HashMap;

pub fn get_score(){
    let a = std::fs::read_to_string("/home/stijn/Projects/Rust_Advent_Of_Code_2022/aoc2022/src/day2/input.txt").unwrap();

    let pts: HashMap<&str, i32>  = HashMap::from([("X",1),("Y",2),("Z",3)]);
    let  m_ix: HashMap<&str, usize> = HashMap::from([
        ("A",0),("B",1),("C",2),
        ("X",0),("Y",1),("Z",2)]);

    let m_pts:[[i32;3];3] = [[3,6,0],
                            [0,3,6],
                            [6,0,3]];
    
    let b =
        a.lines().map(|s| s.split_whitespace().collect::<Vec<&str>>());

    let mut _sum = 0;

    for spl in b{
        _sum += pts.get(spl[1]).unwrap();
        // println!("{}",pts.get(spl[1]).unwrap());
        _sum += m_pts[(*m_ix.get(spl[0]).unwrap())][(*m_ix.get(spl[1]).unwrap())];
        // println!("{}",m_pts[(*m_ix.get(spl[0]).unwrap())][(*m_ix.get(spl[1]).unwrap())]);

    }
    println!("{}",_sum);    


} 